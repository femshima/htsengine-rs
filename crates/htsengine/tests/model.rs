use htsengine::HTSEngine;

const SAMPLE_SENTENCE: [&str;41]= [
    "/A:xx+xx+xx/B:xx-xx_xx/C:xx_xx+xx/D:04+xx_xx/E:xx_xx!xx_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:3_1%0_xx_xx/H:xx_xx/I:xx-xx@xx+xx&xx-xx|xx+xx/J:4_12/K:2+6-21",
    "/A:0+1+3/B:xx-xx_xx/C:04_xx+xx/D:13+xx_xx/E:xx_xx!xx_xx-xx/F:3_1#0_xx@1_4|1_12/G:3_3%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
    "/A:0+1+3/B:xx-xx_xx/C:04_xx+xx/D:13+xx_xx/E:xx_xx!xx_xx-xx/F:3_1#0_xx@1_4|1_12/G:3_3%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
    "/A:1+2+2/B:xx-xx_xx/C:04_xx+xx/D:13+xx_xx/E:xx_xx!xx_xx-xx/F:3_1#0_xx@1_4|1_12/G:3_3%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
    "/A:1+2+2/B:xx-xx_xx/C:04_xx+xx/D:13+xx_xx/E:xx_xx!xx_xx-xx/F:3_1#0_xx@1_4|1_12/G:3_3%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
    "/A:2+3+1/B:04-xx_xx/C:13_xx+xx/D:20+1_1/E:xx_xx!xx_xx-xx/F:3_1#0_xx@1_4|1_12/G:3_3%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
    "/A:-2+1+3/B:13-xx_xx/C:20_1+1/D:12+xx_xx/E:3_1!0_xx-1/F:3_3#0_xx@2_3|4_9/G:2_2%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
    "/A:-1+2+2/B:13-xx_xx/C:20_1+1/D:12+xx_xx/E:3_1!0_xx-1/F:3_3#0_xx@2_3|4_9/G:2_2%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
    "/A:0+3+1/B:20-1_1/C:12_xx+xx/D:17+3_2/E:3_1!0_xx-1/F:3_3#0_xx@2_3|4_9/G:2_2%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
    "/A:0+3+1/B:20-1_1/C:12_xx+xx/D:17+3_2/E:3_1!0_xx-1/F:3_3#0_xx@2_3|4_9/G:2_2%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
    "/A:-1+1+2/B:12-xx_xx/C:17_3+2/D:22+xx_xx/E:3_3!0_xx-1/F:2_2#0_xx@3_2|7_6/G:4_2%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
    "/A:0+2+1/B:12-xx_xx/C:17_3+2/D:22+xx_xx/E:3_3!0_xx-1/F:2_2#0_xx@3_2|7_6/G:4_2%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
    "/A:0+2+1/B:12-xx_xx/C:17_3+2/D:22+xx_xx/E:3_3!0_xx-1/F:2_2#0_xx@3_2|7_6/G:4_2%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
    "/A:-1+1+4/B:17-3_2/C:22_xx+xx/D:10+7_2/E:2_2!0_xx-1/F:4_2#0_xx@4_1|9_4/G:3_3%0_xx_0/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
    "/A:-1+1+4/B:17-3_2/C:22_xx+xx/D:10+7_2/E:2_2!0_xx-1/F:4_2#0_xx@4_1|9_4/G:3_3%0_xx_0/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
    "/A:0+2+3/B:22-xx_xx/C:10_7+2/D:23+xx_xx/E:2_2!0_xx-1/F:4_2#0_xx@4_1|9_4/G:3_3%0_xx_0/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
    "/A:0+2+3/B:22-xx_xx/C:10_7+2/D:23+xx_xx/E:2_2!0_xx-1/F:4_2#0_xx@4_1|9_4/G:3_3%0_xx_0/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
    "/A:1+3+2/B:22-xx_xx/C:10_7+2/D:23+xx_xx/E:2_2!0_xx-1/F:4_2#0_xx@4_1|9_4/G:3_3%0_xx_0/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
    "/A:1+3+2/B:22-xx_xx/C:10_7+2/D:23+xx_xx/E:2_2!0_xx-1/F:4_2#0_xx@4_1|9_4/G:3_3%0_xx_0/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
    "/A:2+4+1/B:10-7_2/C:23_xx+xx/D:04+xx_xx/E:2_2!0_xx-1/F:4_2#0_xx@4_1|9_4/G:3_3%0_xx_0/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
    "/A:2+4+1/B:10-7_2/C:23_xx+xx/D:04+xx_xx/E:2_2!0_xx-1/F:4_2#0_xx@4_1|9_4/G:3_3%0_xx_0/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
    "/A:xx+xx+xx/B:23-xx_xx/C:xx_xx+xx/D:04+xx_xx/E:4_2!0_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:3_3%0_xx_xx/H:4_12/I:xx-xx@xx+xx&xx-xx|xx+xx/J:2_9/K:2+6-21",
    "/A:-2+1+3/B:23-xx_xx/C:04_xx+xx/D:24+xx_xx/E:4_2!0_xx-0/F:3_3#0_xx@1_2|1_9/G:6_4%0_xx_1/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
    "/A:-2+1+3/B:23-xx_xx/C:04_xx+xx/D:24+xx_xx/E:4_2!0_xx-0/F:3_3#0_xx@1_2|1_9/G:6_4%0_xx_1/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
    "/A:-1+2+2/B:23-xx_xx/C:04_xx+xx/D:24+xx_xx/E:4_2!0_xx-0/F:3_3#0_xx@1_2|1_9/G:6_4%0_xx_1/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
    "/A:-1+2+2/B:23-xx_xx/C:04_xx+xx/D:24+xx_xx/E:4_2!0_xx-0/F:3_3#0_xx@1_2|1_9/G:6_4%0_xx_1/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
    "/A:0+3+1/B:04-xx_xx/C:24_xx+xx/D:02+xx_xx/E:4_2!0_xx-0/F:3_3#0_xx@1_2|1_9/G:6_4%0_xx_1/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
    "/A:0+3+1/B:04-xx_xx/C:24_xx+xx/D:02+xx_xx/E:4_2!0_xx-0/F:3_3#0_xx@1_2|1_9/G:6_4%0_xx_1/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
    "/A:-3+1+6/B:24-xx_xx/C:02_xx+xx/D:10+7_2/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
    "/A:-3+1+6/B:24-xx_xx/C:02_xx+xx/D:10+7_2/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
    "/A:-2+2+5/B:24-xx_xx/C:02_xx+xx/D:10+7_2/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
    "/A:-2+2+5/B:24-xx_xx/C:02_xx+xx/D:10+7_2/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
    "/A:-1+3+4/B:24-xx_xx/C:02_xx+xx/D:10+7_2/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
    "/A:-1+3+4/B:24-xx_xx/C:02_xx+xx/D:10+7_2/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
    "/A:0+4+3/B:02-xx_xx/C:10_7+2/D:14+xx_xx/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
    "/A:0+4+3/B:02-xx_xx/C:10_7+2/D:14+xx_xx/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
    "/A:1+5+2/B:02-xx_xx/C:10_7+2/D:14+xx_xx/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
    "/A:1+5+2/B:02-xx_xx/C:10_7+2/D:14+xx_xx/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
    "/A:2+6+1/B:10-7_2/C:14_xx+xx/D:xx+xx_xx/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
    "/A:2+6+1/B:10-7_2/C:14_xx+xx/D:xx+xx_xx/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
    "/A:xx+xx+xx/B:14-xx_xx/C:xx_xx+xx/D:xx+xx_xx/E:6_4!0_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:xx_xx%xx_xx_xx/H:2_9/I:xx-xx@xx+xx&xx-xx|xx+xx/J:xx_xx/K:2+6-21",
];

#[test]
#[ignore]
fn synthesis() {
    let mut engine = HTSEngine::new();

    engine
        .load(vec![
            "tests/models/nitech_jp_atr503_m001.htsvoice".to_string()
        ])
        .unwrap();

    engine.set_sampling_frequency(48000);
    let result = engine
        .synthesize(SAMPLE_SENTENCE.map(|s| s.to_string()).to_vec())
        .unwrap();
    assert_eq!(result.len(), 222720);
}