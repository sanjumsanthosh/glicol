#![cfg(test)]

use glicol_parser::{get_ast, nodes::*, Rule};
use pest::error::{Error, ErrorVariant};

fn ast_from_nodes<const N: usize>(
    nodes: [(&'static str, Vec<Component<'static>>); N],
) -> Result<Ast<'static>, Box<Error<Rule>>> {
    Ok(Ast {
        nodes: hashbrown::HashMap::from_iter(nodes),
    })
}

#[test]
fn points() {
    assert_eq!(
        get_ast("o: points [0 => 0.0, 1/4 => 1.0, 1/2 => 0.5, 3/4 => 1.0, 1 => 0.0]"),
        ast_from_nodes([(
            "o",
            vec![Component::Points(Points {
                points: vec![
                    (TimeList { bar: 0.0, time: None }, 0.0),
                    (TimeList { bar: 0.25, time: None }, 1.0),
                    (TimeList { bar: 0.5, time: None }, 0.5),
                    (TimeList { bar: 0.75, time: None }, 1.0),
                    (TimeList { bar: 1.0, time: None }, 0.0),
                ],
                span: 1.0,
                is_looping: false,
            })]
        )])
    );

    assert_eq!(
        get_ast("o: points [0 => 0.0, 1/4 => 1.0, 1/2 => 0.5, 3/4 => 1.0, 1 => 0.0] * 2"),
        ast_from_nodes([(
            "o",
            vec![Component::Points(Points {
                points: vec![
                    (TimeList { bar: 0.0, time: None }, 0.0),
                    (TimeList { bar: 0.25, time: None }, 1.0),
                    (TimeList { bar: 0.5, time: None }, 0.5),
                    (TimeList { bar: 0.75, time: None }, 1.0),
                    (TimeList { bar: 1.0, time: None }, 0.0),
                ],
                span: 2.0,
                is_looping: false,
            })]
        )])
    );

    assert_eq!(
        get_ast("o: points [0 => 0.0, 1/4 => 1.0, 1/2 => 0.5, 3/4 => 1.0, 1 => 0.0] !"),
        ast_from_nodes([(
            "o",
            vec![Component::Points(Points {
                points: vec![
                    (TimeList { bar: 0.0, time: None }, 0.0),
                    (TimeList { bar: 0.25, time: None }, 1.0),
                    (TimeList { bar: 0.5, time: None }, 0.5),
                    (TimeList { bar: 0.75, time: None }, 1.0),
                    (TimeList { bar: 1.0, time: None }, 0.0),
                ],
                span: 1.0,
                is_looping: true,
            })]
        )])
    );

    assert_eq!(
        get_ast("o: points [0 => 0.0, 1/4 => 1.0, 1/2 => 0.5, 3/4 => 1.0, 1 => 0.0] * 2 !"),
        ast_from_nodes([(
            "o",
            vec![Component::Points(Points {
                points: vec![
                    (TimeList { bar: 0.0, time: None }, 0.0),
                    (TimeList { bar: 0.25, time: None }, 1.0),
                    (TimeList { bar: 0.5, time: None }, 0.5),
                    (TimeList { bar: 0.75, time: None }, 1.0),
                    (TimeList { bar: 1.0, time: None }, 0.0),
                ],
                span: 2.0,
                is_looping: true,
            })]
        )])
    );
}

#[test]
fn delay() {
    assert_eq!(
        get_ast("o: delayn 8"),
        ast_from_nodes([(
            "o",
            vec![Component::Delayn(Delayn {
                param: UsizeOrRef::Usize(8)
            })]
        )])
    );

    assert_eq!(
        get_ast("o: delayn o"),
        ast_from_nodes([(
            "o",
            vec![Component::Delayn(Delayn {
                param: UsizeOrRef::Ref("o")
            })]
        )])
    );

    assert_eq!(
        match get_ast("o: delayn 0.5").unwrap_err().variant {
            ErrorVariant::ParsingError { positives, .. } => positives,
            _ => unreachable!(),
        },
        vec![Rule::integer]
    );

    assert_eq!(
        get_ast("o: delayms 0.5"),
        ast_from_nodes([(
            "o",
            vec![Component::Delayms(Delayms {
                param: NumberOrRef::Number(0.5)
            })]
        )])
    );

    assert_eq!(
        get_ast("o: delayms 5"),
        ast_from_nodes([(
            "o",
            vec![Component::Delayms(Delayms {
                param: NumberOrRef::Number(5.)
            })]
        )])
    );

    assert_eq!(
        get_ast("o: delayms o"),
        ast_from_nodes([(
            "o",
            vec![Component::Delayms(Delayms {
                param: NumberOrRef::Ref("o")
            })]
        )])
    );
}

#[test]
fn waves() {
    assert_eq!(
        get_ast("o: sin 0.5"),
        ast_from_nodes([(
            "o",
            vec![Component::Sin(Sin {
                param: NumberOrRef::Number(0.5)
            })]
        )])
    );

    assert_eq!(
        get_ast("o: sin i"),
        ast_from_nodes([(
            "o",
            vec![Component::Sin(Sin {
                param: NumberOrRef::Ref("i")
            })]
        )])
    );

    assert_eq!(
        get_ast("o: squ 1100.5"),
        ast_from_nodes([(
            "o",
            vec![Component::Squ(Squ {
                param: NumberOrRef::Number(1100.5)
            })]
        )])
    );

    assert_eq!(
        get_ast("o: squ suq"),
        ast_from_nodes([(
            "o",
            vec![Component::Squ(Squ {
                param: NumberOrRef::Ref("suq")
            })]
        )])
    );

    assert_eq!(
        get_ast("o: saw 00.5"),
        ast_from_nodes([(
            "o",
            vec![Component::Saw(Saw {
                param: NumberOrRef::Number(0.5)
            })]
        )])
    );

    assert_eq!(
        get_ast("o: saw ooooo"),
        ast_from_nodes([(
            "o",
            vec![Component::Saw(Saw {
                param: NumberOrRef::Ref("ooooo")
            })]
        )])
    );
}

#[test]
fn seq() {
    assert_eq!(
        get_ast("o: seq 60_ 1000_ 1010__10 _1010_1011_ 1_1_ ~a12_13_ ~r4 4"),
        ast_from_nodes([(
            "o",
            vec![Component::Seq(Seq {
                events: vec![
                    (0., NumberOrRef::Number(60.)),
                    (1., NumberOrRef::Number(1000.)),
                    (2., NumberOrRef::Number(1010.)),
                    (2.75, NumberOrRef::Number(10.)),
                    (3.2, NumberOrRef::Number(1010.)),
                    (3.6, NumberOrRef::Number(1011.)),
                    (4., NumberOrRef::Number(1.)),
                    (4.5, NumberOrRef::Number(1.)),
                    (5., NumberOrRef::Ref("~a")),
                    (5.2, NumberOrRef::Number(12.)),
                    (5.6, NumberOrRef::Number(13.)),
                    (6., NumberOrRef::Ref("~r")),
                    (6.5, NumberOrRef::Number(4.)),
                    (7., NumberOrRef::Number(4.))
                ]
            })]
        )])
    );
}

#[test]
fn arrange() {
    assert_eq!(
        get_ast("o: arrange ~o 1"),
        ast_from_nodes([(
            "o",
            vec![Component::Arrange(Arrange {
                events: vec![NumberOrRef::Ref("~o"), NumberOrRef::Number(1.)]
            })]
        )])
    );

    assert_eq!(
        get_ast("o: arrange ~t1 3 ~t2 1"),
        ast_from_nodes([(
            "o",
            vec![Component::Arrange(Arrange {
                events: vec![
                    NumberOrRef::Ref("~t1"),
                    NumberOrRef::Number(3.),
                    NumberOrRef::Ref("~t2"),
                    NumberOrRef::Number(1.)
                ]
            })]
        )])
    );
}

#[test]
fn choose() {
    assert_eq!(
        get_ast("~a: choose 42 42 42 42 42 37 0 0 0 0"),
        ast_from_nodes([(
            "~a",
            vec![Component::Choose(Choose {
                choices: vec![42., 42., 42., 42., 42., 37., 0., 0., 0., 0.]
            })]
        )])
    );

    assert_eq!(
        get_ast("o: choose 52"),
        ast_from_nodes([("o", vec![Component::Choose(Choose { choices: vec![52.] })])])
    );
}

#[test]
fn mix() {
    assert_eq!(
        get_ast("out: mix ~bd ~sn ~hh ~lead ~basslow ~bassmid"),
        ast_from_nodes([(
            "out",
            vec![Component::Mix(Mix {
                nodes: vec!["~bd", "~sn", "~hh", "~lead", "~basslow", "~bassmid"]
            })]
        )])
    );

    assert_eq!(
        get_ast("out: mix ~t.. ~drum.."),
        ast_from_nodes([(
            "out",
            vec![Component::Mix(Mix {
                nodes: vec!["~t..", "~drum.."]
            })]
        )])
    );
}

#[test]
fn sp() {
    assert_eq!(
        get_ast("o: sp \\808db"),
        ast_from_nodes([(
            "o",
            vec![Component::Sp(Sp {
                sample_sym: "\\808db"
            })]
        )])
    );

    assert_eq!(
        get_ast("o: sp \\guitar"),
        ast_from_nodes([(
            "o",
            vec![Component::Sp(Sp {
                sample_sym: "\\guitar"
            })]
        )])
    );
}

#[test]
fn speed() {
    assert_eq!(
        get_ast("a: speed 16.0"),
        ast_from_nodes([("a", vec![Component::Speed(Speed { speed: 16. })])])
    );
}

#[test]
fn sig() {
    assert_eq!(
        get_ast("fhhfh: sig 4.0"),
        ast_from_nodes([("fhhfh", vec![Component::ConstSig(ConstSig { value: 4.0 })])])
    );

    assert_eq!(
        get_ast("oo_: constsig 5.111"),
        ast_from_nodes([("oo_", vec![Component::ConstSig(ConstSig { value: 5.111 })])])
    );
}

#[test]
fn adc() {
    assert_eq!(
        get_ast("b_b: adc 5"),
        ast_from_nodes([("b_b", vec![Component::Adc(Adc { port: 5 })])])
    );
}

#[test]
fn bd_sn_hh() {
    assert_eq!(
        get_ast("~bd: bd 0.03"),
        ast_from_nodes([(
            "~bd",
            vec![Component::Bd(Bd {
                param: NumberOrRef::Number(0.03)
            })]
        )])
    );

    assert_eq!(
        get_ast("~ssss: sn 0.05"),
        ast_from_nodes([(
            "~ssss",
            vec![Component::Sn(Sn {
                param: NumberOrRef::Number(0.05)
            })]
        )])
    );
}

#[test]
fn synths() {
    assert_eq!(
        get_ast("synthy: sawsynth 0.01 0.3"),
        ast_from_nodes([(
            "synthy",
            vec![Component::SawSynth(SawSynth {
                attack: 0.01,
                decay: 0.3
            })]
        )])
    );

    assert_eq!(
        get_ast("q: squsynth 1.000 300"),
        ast_from_nodes([(
            "q",
            vec![Component::SquSynth(SquSynth {
                attack: 1.,
                decay: 300.
            })]
        )])
    );

    assert_eq!(
        get_ast("i01: trisynth 0.00 9.9"),
        ast_from_nodes([(
            "i01",
            vec![Component::TriSynth(TriSynth {
                attack: 0.,
                decay: 9.9
            })]
        )])
    );
}

#[test]
fn lpf() {
    assert_eq!(
        get_ast("~l: lpf ~mod 1.0"),
        ast_from_nodes([(
            "~l",
            vec![Component::Lpf(Lpf {
                signal: Signal::Reference("~mod"),
                qvalue: 1.
            })]
        )])
    );

    assert_eq!(
        get_ast("ooo: lpf 100.0 1.0"),
        ast_from_nodes([(
            "ooo",
            vec![Component::Lpf(Lpf {
                signal: Signal::Number(100.),
                qvalue: 1.
            })]
        )])
    );
}

#[test]
fn balance() {
    assert_eq!(
        get_ast("o0: balance ~llll right0"),
        ast_from_nodes([(
            "o0",
            vec![Component::Balance(Balance {
                left: "~llll",
                right: "right0"
            })]
        )])
    );
}
