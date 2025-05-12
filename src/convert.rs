use crate::format::Format;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use std::rc::Rc;

/// A function that converts from one format to another.
type ConvertFn = dyn Fn(&mut dyn Read, &mut dyn Write) -> io::Result<()>;

/// A graph of conversion functions
///
/// ```rust
/// use bytary::convert::ConversionGraph;
/// use bytary::format::Format;
///
/// let graph = ConversionGraph::builtins();
/// let conv = graph.get_converter(&Format::Bytes, &Format::Hex).unwrap();
/// ```
pub struct ConversionGraph {
    /// {Format -> {Format -> (ConvertFn, Cost)}}
    graph: HashMap<Format, HashMap<Format, (Rc<ConvertFn>, u32)>>,
}

impl ConversionGraph {
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

    /// Create a new empty graph.
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
    pub fn add_direct<T: Fn(&mut dyn Read, &mut dyn Write) -> io::Result<()> + 'static>(
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

    pub fn get_direct_converter(&self, from: &Format, to: &Format) -> Option<Rc<ConvertFn>> {
        self.graph
            .get(from)
            .and_then(|map| map.get(to))
            .map_or(None, |(f, _)| Some(f.clone()))
    }

    /// Get a converter from `from` to `to`.
    ///
    /// If `to` is equals to `from`, return a converter that simply copies the input.
    pub fn get_converter(&self, from: &Format, to: &Format) -> Option<Rc<ConvertFn>> {
        if to == from {
            return Some(Rc::new(|r, w| io::copy(r, w).map(|_| ())));
        }
        let path = self.find_shortest_path(from, to)?;

        if path.len() <= 1 {
            return None;
        }

        let converters = self.path_to_converters(path);

        Some(Self::compose(converters))
    }

    fn successors(&self, n: &Format) -> Vec<(Format, u32)> {
        self.graph
            .get(&n)
            .unwrap_or(&HashMap::new())
            .iter()
            .map(|(format, (_, cost))| (format.clone(), *cost))
            .collect::<Vec<(Format, u32)>>()
    }

    pub fn find_shortest_path(&self, from: &Format, to: &Format) -> Option<Vec<Format>> {
        Some(dijkstra(from, |n| self.successors(n), |f| f == to)?.0)
    }

    pub fn path_to_converters(&self, path: Vec<Format>) -> Vec<Rc<ConvertFn>> {
        path.windows(2)
            .map(|w| self.get_direct_converter(&w[0], &w[1]).unwrap())
            .collect()
    }
}
