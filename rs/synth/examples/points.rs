use glicol_synth::{operator::Mul, signal::Points, AudioContextBuilder};

fn main() {
    let mut context = AudioContextBuilder::<128>::new()
        .sr(44100)
        .channels(1)
        .build();
    let points = Points::new()
        .points(vec![
            (TimeList { bar: 0.0, time: None }, 0.0),
            (TimeList { bar: 0.25, time: None }, 1.0),
            (TimeList { bar: 0.5, time: None }, 0.5),
            (TimeList { bar: 0.75, time: None }, 1.0),
            (TimeList { bar: 1.0, time: None }, 0.0),
        ])
        .span(1.0)
        .is_looping(true)
        .bpm(120.0)
        .sr(44100);
    let node_a = context.add_mono_node(points);
    let node_b = context.add_mono_node(Mul::new(0.5));
    context.chain(vec![node_a, node_b, context.destination]);

    for _ in 0..(44100 / 128) {
        let buf = context.next_block();
        println!("{:?}", buf[0]);
    }
}
