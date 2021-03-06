#![allow(non_upper_case_globals)]

use graphics;
use graphics::{ Graphics, Context, Transformed };

pub fn draw_character<G: Graphics>(
    polygon: &graphics::Polygon, tween_factor: f64, c: &Context, gl: &mut G
) {
    let d = c.flip_v();
    let d = d.trans(-148.0, -116.0);
    let mut draw = |poly| polygon.draw_tween_lerp(
        poly, 
        tween_factor,
        &d.draw_state,
        d.transform,
        gl
    );
    draw(frames_left_upper_leg);
    draw(frames_left_lower_leg);
    draw(frames_right_upper_leg);
    draw(frames_right_lower_leg);
    draw(frames_body);
    draw(frames_left_arm);
    draw(frames_right_arm);
    draw(frames_head);
}

pub static frames_right_lower_leg: &'static [&'static [[f64; 2]]] = &[
    &[
        [170.0, 213.0],
        [179.0, 218.0],
        [185.0, 226.0],
        [187.0, 237.0],
        [188.0, 252.0],
        [185.0, 267.0],
        [174.0, 277.0],
        [163.0, 267.0],
        [157.0, 252.0],
        [155.0, 241.0],
        [158.0, 227.0],
        [160.0, 219.0]
    ],
    &[
        [206.0, 208.0],
        [212.0, 217.0],
        [215.0, 232.0],
        [208.0, 245.0],
        [200.0, 255.0],
        [187.0, 266.0],
        [175.0, 267.0],
        [170.0, 252.0],
        [173.0, 236.0],
        [178.0, 224.0],
        [185.0, 213.0],
        [194.0, 209.0]
    ],
    &[
        [191.0, 194.0],
        [204.0, 195.0],
        [211.0, 197.0],
        [221.0, 205.0],
        [230.0, 214.0],
        [235.0, 229.0],
        [232.0, 241.0],
        [219.0, 241.0],
        [206.0, 234.0],
        [196.0, 224.0],
        [190.0, 215.0],
        [188.0, 204.0]
    ],
    &[
        [188.0, 215.0],
        [201.0, 213.0],
        [209.0, 220.0],
        [215.0, 232.0],
        [220.0, 245.0],
        [223.0, 263.0],
        [218.0, 278.0],
        [205.0, 275.0],
        [194.0, 265.0],
        [185.0, 250.0],
        [181.0, 239.0],
        [180.0, 227.0]
    ]
];

pub static frames_left_lower_leg: &'static [&'static [[f64; 2]]] = &[
    &[
        [134.0, 217.0],
        [141.0, 223.0],
        [146.0, 232.0],
        [146.0, 245.0],
        [144.0, 256.0],
        [140.0, 269.0],
        [131.0, 273.0],
        [122.0, 266.0],
        [119.0, 253.0],
        [120.0, 240.0],
        [123.0, 227.0],
        [126.0, 220.0]
    ],
    &[
        [107.0, 212.0],
        [118.0, 210.0],
        [130.0, 216.0],
        [136.0, 226.0],
        [145.0, 237.0],
        [148.0, 251.0],
        [142.0, 263.0],
        [132.0, 263.0],
        [121.0, 258.0],
        [110.0, 250.0],
        [101.0, 240.0],
        [98.0, 223.0]
    ],
    &[
        [115.0, 210.0],
        [119.0, 220.0],
        [115.0, 233.0],
        [108.0, 239.0],
        [94.0, 248.0],
        [77.0, 251.0],
        [63.0, 249.0],
        [66.0, 234.0],
        [75.0, 221.0],
        [83.0, 214.0],
        [94.0, 209.0],
        [104.0, 207.0]
    ],
    &[
        [114.0, 220.0],
        [121.0, 230.0],
        [123.0, 244.0],
        [117.0, 255.0],
        [111.0, 266.0],
        [101.0, 275.0],
        [94.0, 277.0],
        [87.0, 268.0],
        [89.0, 251.0],
        [90.0, 241.0],
        [95.0, 231.0],
        [103.0, 224.0]
    ]
];

pub static frames_right_upper_leg: &'static [&'static [[f64; 2]]] = &[
    &[
        [162.0, 170.0],
        [173.0, 172.0],
        [183.0, 185.0],
        [185.0, 198.0],
        [186.0, 209.0],
        [181.0, 224.0],
        [172.0, 229.0],
        [163.0, 226.0],
        [157.0, 218.0],
        [154.0, 207.0],
        [152.0, 191.0],
        [154.0, 179.0]
    ],
    &[
        [163.0, 167.0],
        [174.0, 164.0],
        [187.0, 167.0],
        [198.0, 174.0],
        [205.0, 182.0],
        [212.0, 196.0],
        [207.0, 210.0],
        [196.0, 213.0],
        [183.0, 208.0],
        [174.0, 200.0],
        [164.0, 191.0],
        [159.0, 178.0]
    ],
    &[
        [161.0, 173.0],
        [173.0, 168.0],
        [188.0, 171.0],
        [196.0, 180.0],
        [202.0, 188.0],
        [204.0, 197.0],
        [202.0, 206.0],
        [193.0, 210.0],
        [182.0, 207.0],
        [174.0, 201.0],
        [164.0, 193.0],
        [159.0, 181.0]
    ],
    &[
        [157.0, 174.0],
        [172.0, 175.0],
        [182.0, 184.0],
        [190.0, 192.0],
        [197.0, 199.0],
        [200.0, 213.0],
        [194.0, 225.0],
        [183.0, 226.0],
        [170.0, 221.0],
        [161.0, 210.0],
        [157.0, 198.0],
        [154.0, 185.0]
    ]
];

pub static frames_left_upper_leg: &'static [&'static [[f64; 2]]] = &[
    &[
        [136.0, 180.0],
        [143.0, 189.0],
        [148.0, 198.0],
        [147.0, 211.0],
        [145.0, 219.0],
        [140.0, 227.0],
        [133.0, 232.0],
        [124.0, 228.0],
        [118.0, 216.0],
        [117.0, 207.0],
        [119.0, 194.0],
        [125.0, 183.0]
    ],
    &[
        [134.0, 176.0],
        [139.0, 186.0],
        [140.0, 197.0],
        [136.0, 208.0],
        [128.0, 219.0],
        [118.0, 225.0],
        [107.0, 227.0],
        [99.0, 220.0],
        [98.0, 206.0],
        [103.0, 192.0],
        [111.0, 183.0],
        [121.0, 177.0]
    ],
    &[
        [143.0, 179.0],
        [147.0, 191.0],
        [147.0, 202.0],
        [138.0, 214.0],
        [129.0, 221.0],
        [118.0, 223.0],
        [108.0, 221.0],
        [104.0, 212.0],
        [105.0, 199.0],
        [110.0, 188.0],
        [118.0, 180.0],
        [130.0, 177.0]
    ],
    &[
        [138.0, 179.0],
        [143.0, 189.0],
        [145.0, 200.0],
        [139.0, 210.0],
        [135.0, 219.0],
        [127.0, 225.0],
        [114.0, 222.0],
        [107.0, 215.0],
        [106.0, 205.0],
        [110.0, 195.0],
        [117.0, 187.0],
        [127.0, 180.0]
    ]
];

pub static frames_right_arm: &'static [&'static [[f64; 2]]] = &[
    &[
        [172.0, 131.0],
        [163.0, 123.0],
        [161.0, 107.0],
        [160.0, 96.0],
        [162.0, 86.0],
        [161.0, 67.0],
        [163.0, 54.0],
        [166.0, 41.0],
        [174.0, 37.0],
        [183.0, 43.0],
        [187.0, 54.0],
        [187.0, 69.0],
        [187.0, 86.0],
        [187.0, 99.0],
        [185.0, 112.0],
        [181.0, 125.0]
    ],
    &[
        [168.0, 134.0],
        [165.0, 126.0],
        [170.0, 112.0],
        [179.0, 98.0],
        [187.0, 90.0],
        [194.0, 83.0],
        [205.0, 74.0],
        [216.0, 70.0],
        [227.0, 74.0],
        [230.0, 83.0],
        [227.0, 93.0],
        [219.0, 103.0],
        [208.0, 115.0],
        [200.0, 123.0],
        [190.0, 131.0],
        [180.0, 137.0]
    ],
    &[
        [156.0, 142.0],
        [164.0, 136.0],
        [178.0, 135.0],
        [193.0, 139.0],
        [204.0, 145.0],
        [212.0, 149.0],
        [223.0, 155.0],
        [233.0, 164.0],
        [237.0, 175.0],
        [227.0, 179.0],
        [217.0, 179.0],
        [205.0, 175.0],
        [192.0, 170.0],
        [181.0, 167.0],
        [167.0, 159.0],
        [158.0, 151.0]
    ],
    &[
        [163.0, 130.0],
        [168.0, 128.0],
        [176.0, 128.0],
        [184.0, 131.0],
        [189.0, 134.0],
        [191.0, 140.0],
        [193.0, 144.0],
        [193.0, 153.0],
        [190.0, 161.0],
        [186.0, 166.0],
        [178.0, 166.0],
        [170.0, 163.0],
        [162.0, 157.0],
        [159.0, 149.0],
        [158.0, 141.0],
        [160.0, 137.0]
    ]
];

pub static frames_left_arm: &'static [&'static [[f64; 2]]] = &[
    &[
        [128.0, 145.0],
        [115.0, 141.0],
        [109.0, 125.0],
        [108.0, 109.0],
        [109.0, 93.0],
        [108.0, 78.0],
        [106.0, 65.0],
        [108.0, 50.0],
        [115.0, 42.0],
        [126.0, 46.0],
        [132.0, 58.0],
        [134.0, 74.0],
        [133.0, 90.0],
        [134.0, 103.0],
        [132.0, 120.0],
        [131.0, 134.0]
    ],
    &[
        [126.0, 139.0],
        [113.0, 139.0],
        [101.0, 132.0],
        [93.0, 124.0],
        [88.0, 118.0],
        [79.0, 109.0],
        [72.0, 94.0],
        [67.0, 81.0],
        [71.0, 66.0],
        [80.0, 65.0],
        [89.0, 71.0],
        [99.0, 84.0],
        [106.0, 95.0],
        [111.0, 103.0],
        [122.0, 115.0],
        [127.0, 128.0]
    ],
    &[
        [136.0, 145.0],
        [133.0, 154.0],
        [128.0, 162.0],
        [119.0, 171.0],
        [105.0, 178.0],
        [99.0, 185.0],
        [87.0, 193.0],
        [72.0, 198.0],
        [61.0, 195.0],
        [57.0, 183.0],
        [64.0, 169.0],
        [77.0, 158.0],
        [86.0, 152.0],
        [96.0, 145.0],
        [109.0, 140.0],
        [126.0, 138.0]
    ],
    &[
        [139.0, 127.0],
        [143.0, 134.0],
        [143.0, 141.0],
        [141.0, 149.0],
        [136.0, 156.0],
        [133.0, 161.0],
        [127.0, 164.0],
        [119.0, 167.0],
        [111.0, 165.0],
        [107.0, 159.0],
        [105.0, 152.0],
        [106.0, 145.0],
        [110.0, 135.0],
        [114.0, 131.0],
        [122.0, 126.0],
        [130.0, 125.0]
    ]
];

pub static frames_body: &'static [&'static [[f64; 2]]] = &[
    &[
        [149.0, 131.0],
        [165.0, 134.0],
        [175.0, 144.0],
        [180.0, 151.0],
        [183.0, 164.0],
        [181.0, 182.0],
        [173.0, 194.0],
        [163.0, 201.0],
        [151.0, 203.0],
        [137.0, 199.0],
        [125.0, 191.0],
        [118.0, 179.0],
        [117.0, 169.0],
        [116.0, 150.0]
    ],
    &[
        [146.0, 127.0],
        [163.0, 133.0],
        [176.0, 138.0],
        [182.0, 146.0],
        [185.0, 155.0],
        [184.0, 168.0],
        [180.0, 182.0],
        [168.0, 192.0],
        [154.0, 194.0],
        [134.0, 190.0],
        [120.0, 181.0],
        [115.0, 174.0],
        [113.0, 161.0],
        [115.0, 148.0],
        [122.0, 137.0],
        [133.0, 132.0]
    ],
    &[
        [149.0, 121.0],
        [162.0, 127.0],
        [171.0, 136.0],
        [177.0, 147.0],
        [179.0, 159.0],
        [177.0, 174.0],
        [172.0, 187.0],
        [166.0, 191.0],
        [156.0, 196.0],
        [146.0, 196.0],
        [134.0, 192.0],
        [122.0, 176.0],
        [119.0, 160.0],
        [122.0, 146.0],
        [127.0, 133.0],
        [137.0, 124.0]
    ],
    &[
        [149.0, 115.0],
        [160.0, 123.0],
        [167.0, 133.0],
        [170.0, 142.0],
        [170.0, 152.0],
        [172.0, 168.0],
        [172.0, 185.0],
        [163.0, 195.0],
        [151.0, 197.0],
        [135.0, 195.0],
        [125.0, 183.0],
        [122.0, 170.0],
        [123.0, 156.0],
        [127.0, 141.0],
        [133.0, 130.0],
        [140.0, 121.0]
    ]
];

pub static frames_head: &'static [&'static [[f64; 2]]] = &[
    &[
        [147.0, 100.0],
        [156.0, 103.0],
        [161.0, 112.0],
        [163.0, 122.0],
        [158.0, 128.0],
        [150.0, 131.0],
        [140.0, 130.0],
        [133.0, 124.0],
        [132.0, 115.0],
        [133.0, 108.0],
        [138.0, 103.0],
    ],
    &[
        [146.0, 102.0],
        [153.0, 105.0],
        [160.0, 110.0],
        [163.0, 119.0],
        [162.0, 127.0],
        [156.0, 133.0],
        [146.0, 137.0],
        [138.0, 133.0],
        [129.0, 127.0],
        [126.0, 117.0],
        [130.0, 109.0],
        [136.0, 104.0],
    ],
    &[
        [148.0, 102.0],
        [157.0, 106.0],
        [165.0, 112.0],
        [169.0, 118.0],
        [167.0, 126.0],
        [160.0, 133.0],
        [151.0, 136.0],
        [142.0, 132.0],
        [133.0, 125.0],
        [132.0, 117.0],
        [136.0, 108.0],
        [141.0, 103.0],
    ],
    &[
        [147.0, 103.0],
        [156.0, 106.0],
        [163.0, 112.0],
        [166.0, 116.0],
        [167.0, 127.0],
        [162.0, 133.0],
        [152.0, 137.0],
        [144.0, 132.0],
        [138.0, 126.0],
        [133.0, 119.0],
        [134.0, 112.0],
        [139.0, 105.0],
    ]
];
