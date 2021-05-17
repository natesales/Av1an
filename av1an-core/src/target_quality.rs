use crate::Encoder;
use std::str::FromStr;

pub fn construct_target_quality_command(
  encoder: Encoder,
  threads: String,
  q: String,
) -> Vec<String> {
  match encoder {
    Encoder::aom => vec![
      "aomenc".into(),
      "--passes=1".into(),
      format!("--threads={}", threads),
      "--tile-columns=2".into(),
      "--tile-rows=1".into(),
      "--end-usage=q".into(),
      "-b".into(),
      "8".into(),
      "--cpu-used=6".into(),
      format!("--cq-level={}", q),
      "--enable-filter-intra=0".into(),
      "--enable-smooth-intra=0".into(),
      "--enable-paeth-intra=0".into(),
      "--enable-cfl-intra=0".into(),
      "--enable-obmc=0".into(),
      "--enable-palette=0".into(),
      "--enable-overlay=0".into(),
      "--enable-intrabc=0".into(),
      "--enable-angle-delta=0".into(),
      "--reduced-tx-type-set=1".into(),
      "--enable-dual-filter=0".into(),
      "--enable-intra-edge-filter=0".into(),
      "--enable-order-hint=0".into(),
      "--enable-flip-idtx=0".into(),
      "--enable-dist-wtd-comp=0".into(),
      "--enable-interintra-wedge=0".into(),
      "--enable-onesided-comp=0".into(),
      "--enable-interintra-comp=0".into(),
      "--enable-global-motion=0".into(),
      "--enable-cdef=0".into(),
      "--max-reference-frames=3".into(),
      "--cdf-update-mode=2".into(),
      "--deltaq-mode=0".into(),
      "--sb-size=64".into(),
      "--min-partition-size=32".into(),
      "--max-partition-size=32".into(),
    ],
    Encoder::rav1e => vec![
      "rav1e".into(),
      "-y".into(),
      "-s".into(),
      "10".into(),
      "--threads".into(),
      format!("--threads={}", threads),
      "--tiles".into(),
      "16".into(),
      "--quantizer".into(),
      format!("{}", q),
      "--low-latency".into(),
      "--rdo-lookahead-frames".into(),
      "5".into(),
      "--no-scene-detection".into(),
    ],
    Encoder::libvpx => vec![
      "vpxenc".into(),
      "-b".into(),
      "10".into(),
      "--profile=2".into(),
      "--passes=1".into(),
      "--pass=1".into(),
      "--codec=vp9".into(),
      format!("--threads={}", threads),
      "--cpu-used=9".into(),
      "--end-usage=q".into(),
      format!("--cq-level={}", q),
      "--row-mt=1".into(),
    ],
    Encoder::svt_av1 => vec![
      "SvtAv1EncApp".into(),
      "-i".into(),
      "stdin".into(),
      "--lp".into(),
      format!("{}", threads),
      "--preset".into(),
      "8".into(),
      "--keyint".into(),
      "240".into(),
      "--crf".into(),
      format!("{}", q),
      "--tile-rows".into(),
      "1".into(),
      "--tile-columns".into(),
      "2".into(),
      "--pred-struct".into(),
      "0".into(),
      "--sg-filter-mode".into(),
      "0".into(),
      "--enable-restoration-filtering".into(),
      "0".into(),
      "--cdef-level".into(),
      "0".into(),
      "--disable-dlf".into(),
      "0".into(),
      "--mrp-level".into(),
      "0".into(),
      "--enable-mfmv".into(),
      "0".into(),
      "--enable-local-warp".into(),
      "0".into(),
      "--enable-global-motion".into(),
      "0".into(),
      "--enable-interintra-comp".into(),
      "0".into(),
      "--obmc-level".into(),
      "0".into(),
      "--rdoq-level".into(),
      "0".into(),
      "--filter-intra-level".into(),
      "0".into(),
      "--enable-intra-edge-filter".into(),
      "0".into(),
      "--enable-pic-based-rate-est".into(),
      "0".into(),
      "--pred-me".into(),
      "0".into(),
      "--bipred-3x3".into(),
      "0".into(),
      "--compound".into(),
      "0".into(),
      "--ext-block".into(),
      "0".into(),
      "--hbd-md".into(),
      "0".into(),
      "--palette-level".into(),
      "0".into(),
      "--umv".into(),
      "0".into(),
      "--tf-level".into(),
      "3".into(),
    ],
    Encoder::svt_vp9 => vec![
      "SvtVp9EncApp".into(),
      "-i".into(),
      "stdin".into(),
      "--lp".into(),
      format!("{}", threads),
      "-enc-mode".into(),
      "8".into(),
      "-q".into(),
      format!("{}", q),
    ],
    Encoder::x264 => vec![
      "x264".into(),
      "--log-level".into(),
      "error".into(),
      "--demuxer".into(),
      "y4m".into(),
      "-".into(),
      "--no-progress".into(),
      "--threads".into(),
      format!("{}", threads),
      "--preset".into(),
      "medium".into(),
      "--crf".into(),
      format!("{}", q),
    ],
    Encoder::x265 => vec![
      "x265".into(),
      "--log-level".into(),
      "0".into(),
      "--no-progress".into(),
      "--y4m".into(),
      "--frame-threads".into(),
      format!("{}", threads),
      "--preset".into(),
      "fast".into(),
      "--crf".into(),
      format!("{}", q),
    ],
    _ => vec!["".into()],
  }
}