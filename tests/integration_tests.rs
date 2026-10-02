//dankje gemini
use manhattan_sweepline::*;

#[test]
fn test_sweep_line_simple() {
    let mut sweep_line = SweepLine::default();

    sweep_line.add_line(Line {
        id: 1,
        start: Point { x: 1.0, y: 1.0 },
        end: Point { x: 5.0, y: 1.0 },
    });

    sweep_line.add_line(Line {
        id: 2,
        start: Point { x: 2.0, y: 2.0 },
        end: Point { x: 4.0, y: 2.0 },
    });

    let res = sweep_line.run();
    assert_eq!(res, 6.0);
}
#[test]
fn test_sweep_line_complex() {
    let mut sweep_line = SweepLine::default();

    // Lijn 1: Brede basislijn
    sweep_line.add_line(Line {
        id: 1,
        start: Point { x: 0.0, y: 2.0 },
        end: Point { x: 10.0, y: 2.0 },
    });

    // Lijn 2: Middelhoge plateau dat start na de basislijn
    // en stopt voor de eerste grote piek
    sweep_line.add_line(Line {
        id: 2,
        start: Point { x: 1.0, y: 3.0 },
        end: Point { x: 5.0, y: 3.0 },
    });

    // Lijn 3: Hoge piek die volledig bovenop Lijn 2 ligt
    sweep_line.add_line(Line {
        id: 3,
        start: Point { x: 2.0, y: 5.0 },
        end: Point { x: 4.0, y: 5.0 },
    });

    // Lijn 4: Hoogste piek in het tweede deel van de basislijn
    sweep_line.add_line(Line {
        id: 4,
        start: Point { x: 6.0, y: 6.0 },
        end: Point { x: 8.0, y: 6.0 },
    });

    // Lijn 5: Lijn die start onder de hoogste piek (Lijn 4),
    // maar langer doorloopt op de x-as
    sweep_line.add_line(Line {
        id: 5,
        start: Point { x: 7.0, y: 4.0 },
        end: Point { x: 9.0, y: 4.0 },
    });

    // Lijn 6: Geïsoleerde lijn na een gap om te testen of
    // lege ruimtes op de x-as correct (als 0) berekend worden
    sweep_line.add_line(Line {
        id: 6,
        start: Point { x: 12.0, y: 3.0 },
        end: Point { x: 14.0, y: 3.0 },
    });

    let res = sweep_line.run();

    // De totale berekende oppervlakte (area under the skyline) is 44.0
    // Berekening per x-interval:
    // [0, 1] : breedte 1 * hoogte 2 (L1) = 2.0
    // [1, 2] : breedte 1 * hoogte 3 (L2) = 3.0
    // [2, 4] : breedte 2 * hoogte 5 (L3) = 10.0
    // [4, 5] : breedte 1 * hoogte 3 (L2) = 3.0
    // [5, 6] : breedte 1 * hoogte 2 (L1) = 2.0
    // [6, 7] : breedte 1 * hoogte 6 (L4) = 6.0
    // [7, 8] : breedte 1 * hoogte 6 (L4) = 6.0
    // [8, 9] : breedte 1 * hoogte 4 (L5) = 4.0
    // [9, 10]: breedte 1 * hoogte 2 (L1) = 2.0
    // [10, 12]: breedte 2 * hoogte 0 (gap) = 0.0
    // [12, 14]: breedte 2 * hoogte 3 (L6) = 6.0
    // Totaal: 2 + 3 + 10 + 3 + 2 + 6 + 6 + 4 + 2 + 0 + 6 = 44.0

    assert_eq!(res, 44.0);
}
#[test]
fn test_sweep_line_reversed_coordinates() {
    let mut sweep_line = SweepLine::default();

    // De x-waarde van start is groter dan end, dit moet intern geswapt worden.
    sweep_line.add_line(Line {
        id: 1,
        start: Point { x: 6.0, y: 4.0 },
        end: Point { x: 2.0, y: 4.0 },
    });

    let res = sweep_line.run();
    // Breedte: 6.0 - 2.0 = 4.0
    // Hoogte: 4.0
    // Oppervlakte: 4.0 * 4.0 = 16.0
    assert_eq!(res, 16.0);
}
#[test]
fn test_sweep_line_identical_lines() {
    let mut sweep_line = SweepLine::default();

    sweep_line.add_line(Line {
        id: 1,
        start: Point { x: 1.0, y: 5.0 },
        end: Point { x: 5.0, y: 5.0 },
    });

    // Identiek aan lijn 1, met uitzondering van het id
    sweep_line.add_line(Line {
        id: 2,
        start: Point { x: 1.0, y: 5.0 },
        end: Point { x: 5.0, y: 5.0 },
    });

    let res = sweep_line.run();
    // Ze bedekken exact dezelfde ruimte, dus de maximale hoogte blijft 5.0
    // en de oppervlakte overlapt niet extra.
    // 4.0 * 5.0 = 20.0
    assert_eq!(res, 20.0);
}
#[test]
fn test_sweep_line_zero_width() {
    let mut sweep_line = SweepLine::default();

    sweep_line.add_line(Line {
        id: 1,
        start: Point { x: 3.0, y: 10.0 },
        end: Point { x: 3.0, y: 10.0 },
    });

    sweep_line.add_line(Line {
        id: 2,
        start: Point { x: 5.0, y: 5.0 },
        end: Point { x: 5.0, y: 5.0 },
    });

    let res = sweep_line.run();
    // Omdat dx (current_x - start_x) altijd 0 is, moet het resultaat 0 zijn.
    assert_eq!(res, 0.0);
}
#[test]
fn test_sweep_line_staircase() {
    let mut sweep_line = SweepLine::default();

    // Basis
    sweep_line.add_line(Line {
        id: 1,
        start: Point { x: 0.0, y: 1.0 },
        end: Point { x: 4.0, y: 1.0 },
    });
    // Midden trede
    sweep_line.add_line(Line {
        id: 2,
        start: Point { x: 1.0, y: 2.0 },
        end: Point { x: 3.0, y: 2.0 },
    });
    // Top
    sweep_line.add_line(Line {
        id: 3,
        start: Point { x: 1.5, y: 3.0 },
        end: Point { x: 2.5, y: 3.0 },
    });

    let res = sweep_line.run();

    // Oppervlaktes per segment:
    // [0.0, 1.0]: breedte 1.0 * hoogte 1.0 = 1.0
    // [1.0, 1.5]: breedte 0.5 * hoogte 2.0 = 1.0
    // [1.5, 2.5]: breedte 1.0 * hoogte 3.0 = 3.0
    // [2.5, 3.0]: breedte 0.5 * hoogte 2.0 = 1.0
    // [3.0, 4.0]: breedte 1.0 * hoogte 1.0 = 1.0
    // Totaal: 1.0 + 1.0 + 3.0 + 1.0 + 1.0 = 7.0
    assert_eq!(res, 7.0);
}