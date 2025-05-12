use crate::error::BytaryResult;
use crate::format::Format;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use std::rc::Rc;

/// A function that converts from one format to another.
type ConvertFn = dyn Fn(&mut dyn Read, &mut dyn Write) -> BytaryResult<()>;

/// A graph of conversion functions
///
/// ```rust
/// use bytary::convert::ConversionGraph;
/// use bytary::format::Format::*;
///
/// let graph = ConversionGraph::default();
/// let conv = graph.get_converter(&Bytes, &Hex).unwrap();
/// ```
pub struct ConversionGraph {
    /// {Format -> {Format -> (ConvertFn, Cost)}}
    graph: HashMap<Format, HashMap<Format, (Rc<ConvertFn>, u32)>>,
}

impl ConversionGraph {
    /// Compose a list of converters into a single converter.
    pub fn compose(converters: Vec<Rc<ConvertFn>>) -> Rc<ConvertFn> {
        if converters.len() == 1 {
            return converters[0].clone();
        }

        Rc::new(move |input: &mut dyn Read, output: &mut dyn Write| {
            let mut prev_output: Box<dyn Read> = Box::new(input);

            for converter in converters.iter().take(converters.len().saturating_sub(1)) {
                let mut buffer = Vec::new();
                converter(&mut prev_output, &mut buffer)?;
                prev_output = Box::new(io::Cursor::new(buffer));
            }

            if let Some(last_processor) = converters.last() {
                last_processor(&mut prev_output, output)?;
            }

            Ok(())
        })
    }

    /// Get a converter that copies the input to the output without any conversion.
    pub fn get_copy_converter() -> Rc<ConvertFn> {
        Rc::new(|r, w| {
            io::copy(r, w)?;
            Ok(())
        })
    }
    /// Create a new empty [`ConversionGraph`]
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
        }
    }
    /// Returns the number of conversions in the graph
    pub fn size(&self) -> usize {
        self.graph.iter().map(|(_, h)| h.len()).sum()
    }
    /// Adds a direct conversion to the graph
    pub fn add_direct<T: Fn(&mut dyn Read, &mut dyn Write) -> BytaryResult<()> + 'static>(
        &mut self,
        from: Format,
        to: Format,
        converter: T,
        cost: u32,
    ) {
        self.graph
            .entry(from)
            .or_default()
            .insert(to, (Rc::new(converter), cost));
    }
    /// Get a converter from `from` to `to`.
    ///
    /// If `to` is equals to `from`, return a converter that simply copies the input.
    pub fn get_converter(&self, from: &Format, to: &Format) -> Option<Rc<ConvertFn>> {
        if to == from {
            return Some(Self::get_copy_converter());
        }
        let path = self.find_shortest_path(from, to)?;

        if path.len() <= 1 {
            return None;
        }

        let converters = self.path_to_converters(&path).unwrap();

        Some(Self::compose(converters))
    }
    /// ```rust
    /// use bytary::convert::ConversionGraph;
    /// use bytary::format::Format::*;
    ///
    /// let mut graph = ConversionGraph::new();
    ///
    /// graph.add_direct(Bytes, Hex, |_,_| Ok(()), 1);
    /// assert!(graph.can_convert(&Bytes, &Hex));
    /// assert!(!graph.can_convert(&Hex, &Bytes));
    /// ```
    pub fn can_convert(&self, from: &Format, to: &Format) -> bool {
        if from == to {
            return true;
        }
        self.find_shortest_path(from, to).is_some()
    }
    /// ```rust
    /// use bytary::convert::ConversionGraph;
    /// use bytary::format::Format::*;
    ///
    /// let mut graph = ConversionGraph::new();
    ///
    /// graph.add_direct(Bytes, Hex, |_,_| Ok(()), 1);
    /// assert!(!graph.can_convert_between(&Bytes, &Hex));
    ///
    /// graph.add_direct(Hex, Bytes, |_,_| Ok(()), 1);
    /// assert!(graph.can_convert_between(&Bytes, &Hex));
    /// ```
    pub fn can_convert_between(&self, format1: &Format, format2: &Format) -> bool {
        if format1 == format2 {
            return true;
        }
        self.find_shortest_path(format1, format2)
            .and(self.find_shortest_path(format2, format1))
            .is_some()
    }

    /// Finds the shortest path between two formats.
    ///
    /// Returns a vector of formats representing the shortest path, or `None` if no path exists.
    pub fn find_shortest_path(&self, from: &Format, to: &Format) -> Option<Vec<Format>> {
        Some(dijkstra(from, |n| self.successors(n), |f| f == to)?.0)
    }

    /// Get converters from given path
    ///
    /// ## Params
    ///
    /// - `path`: A vector of formats representing the path. The first format is the origin format to convert from, the last format is the destination format to convert to finally.
    ///
    /// ## Returns
    ///
    /// A vector of converter functions. If any converter between formats is not found, it returns `None`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use bytary::convert::ConversionGraph;
    /// use bytary::format::Format::*;
    ///
    /// let graph = ConversionGraph::default();
    /// let converters = graph.path_to_converters(&vec![Bytes, Bin, Hex]).unwrap();
    /// ```
    ///
    /// The result `converters` will contain two converter functions.
    ///
    /// 1. Converts bytes to binary representation.
    /// 2. Converts binary to hexadecimal representation.
    pub fn path_to_converters(&self, path: &Vec<Format>) -> Option<Vec<Rc<ConvertFn>>> {
        let converters = path
            .windows(2)
            .map_while(|w| Some(self.get_direct_converter(&w[0], &w[1])?))
            .collect();
        Some(converters)
    }

    fn get_direct_converter(&self, from: &Format, to: &Format) -> Option<Rc<ConvertFn>> {
        self.graph
            .get(from)
            .and_then(|map| map.get(to))
            .map_or(None, |(f, _)| Some(f.clone()))
    }
    fn successors(&self, n: &Format) -> Vec<(Format, u32)> {
        self.graph
            .get(&n)
            .unwrap_or(&HashMap::new())
            .iter()
            .map(|(format, (_, cost))| (format.clone(), *cost))
            .collect::<Vec<(Format, u32)>>()
    }
}
