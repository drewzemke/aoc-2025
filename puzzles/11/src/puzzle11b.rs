use common::puzzle::PuzzlePart;

use crate::Graph;

pub struct Puzzle11b {}

impl PuzzlePart for Puzzle11b {
    fn description() -> &'static str {
        "Find the number of paths in a network that pass through two particular nodes"
    }

    fn solve(input: &str) -> String {
        let graph = Graph::parse(input);

        let svr = graph.find_node("svr");
        let fft = graph.find_node("fft");
        let dac = graph.find_node("dac");
        let out = graph.find_node("out");

        // if a path passes through both fft and dac, then it either looks like
        //   svr -- fft -- dac -- out
        //  or
        //   svr -- dac -- fft -- out
        //
        // ... can we just count segments and take products and out?
        let svr_to_fft = graph.count_paths(svr, fft);
        let fft_to_dac = graph.count_paths(fft, dac);
        let dac_to_out = graph.count_paths(dac, out);

        let svr_to_dac = graph.count_paths(svr, dac);
        let dac_to_fft = graph.count_paths(dac, fft);
        let fft_to_out = graph.count_paths(fft, out);

        ((svr_to_fft * fft_to_dac * dac_to_out) + (svr_to_dac * dac_to_fft * fft_to_out))
            .to_string()
    }
}
