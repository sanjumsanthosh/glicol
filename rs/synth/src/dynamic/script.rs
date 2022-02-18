use dasp_graph::{Buffer, Input, Node};
use dasp_ring_buffer as ring_buffer;
use dasp_signal::{self as signal, Signal};
use dasp_interpolate::linear::Linear;
use dasp_interpolate::sinc::Sinc;
use rhai::{Engine, Array, Scope, Dynamic, EvalAltResult, OptimizationLevel};
use super::super::{GlicolNodeData, NodeData, BoxedNodeSend, mono_node};
type Fixed = ring_buffer::Fixed<Vec<f32>>;

pub struct Script<const N: usize> {
    buf: Fixed,
    sr: usize,
    code: String,
    scope: Scope<'static>,
    engine: Engine,
    phase: usize,
}

impl<const N: usize> Script<N> {
    pub fn new() -> Self {
        let mut phase: usize = 0;
        let mut scope = Scope::new();
        scope.push("phase", phase as f32).push("output", Array::new());
        let mut engine = Engine::new();
        engine.set_optimization_level(OptimizationLevel::Full);
        Self { 
            buf: ring_buffer::Fixed::from(vec![0.0]), 
            sr: 44100, 
            code: "21.0*2.0".to_owned(),
            engine,
            scope,
            phase,
        }
    }
    pub fn sr(self, sr:usize) -> Self {
        Self {sr, ..self}
    }

    pub fn code(self, code: String) -> Self {
        Self {code, ..self}
    }

    pub fn build(self) -> GlicolNodeData<N> {

        mono_node!( N, self)
    }
}

impl<const N: usize> Node<N> for Script<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        // let l = inputs.len();
        // let has_clock;
        // if l >= 1 { 
        //     has_clock = inputs[l-1].buffers()[0][0] as usize % N == 0
        //     && inputs[l-1].buffers()[0][1] == 0.;
        // } else {
        //     has_clock = false;
        // }
        let mut arr = Array::new();
        for i in 0..N {
            arr.push(Dynamic::from_float(inputs[0].buffers()[0][i]));
        }
        self.scope.set_or_push("input", arr);
        let result: Array = self.engine.eval_with_scope(&mut self.scope, &self.code).unwrap();

        for i in 0..N {
            output[0][i] = result[i].as_float().unwrap();
        }
        self.phase += N;
    }
   
}