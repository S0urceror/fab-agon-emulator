const HELP: &str = "\
The Fabulous Agon Emulator!

USAGE:
  fab-agon-emulator [OPTIONS]

OPTIONS:
  -d, --debugger        Enable the eZ80 debugger
  -b, --breakpoint      Set a breakpoint before starting
  -z, --zero            Initialize ram with zeroes instead of random values
  -f, --fullscreen      Start in fullscreen mode
  -h, --help            Prints help information
  -u, --unlimited-cpu   Don't limit eZ80 CPU frequency
  --firmware 1.03       Use quark 1.03 firmware (default is console8)
  --firmware quark      Use quark 1.04 firmware (default is console8)
  --firmware electron   Use ElectronOS firmware (default is console8)
  --sdcard <path>       Sets the path of the emulated SDCard
  --scale <max-height>  Use perfect (integer) video mode scaling, up to
                        a maximum window height of <max-height>

ADVANCED:
  --mos PATH            Use a different MOS.bin firmware
  --vdp PATH            Use a different VDP dll/so firmware
  --renderer sw         Use software renderer
  --renderer hw         Use GL/D3D renderer (default)
";

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum FirmwareVer {
  quark103,
  quark,
  console8,
  electronhal
}

#[derive(Debug)]
pub enum Renderer {
  Software,
  Accelerated
}

#[derive(Debug)]
pub struct AppArgs {
    pub sdcard: Option<String>,
    pub debugger: bool,
    pub breakpoint: Option<String>,
    pub unlimited_cpu: bool,
    pub fullscreen: bool,
    pub zero: bool,
    pub mos_bin: Option<std::path::PathBuf>,
    pub vdp_dll: Option<std::path::PathBuf>,
    pub firmware: FirmwareVer,
    pub perfect_scale: Option<u32>,
    pub renderer: Renderer,
}

pub fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let firmware_ver: Option<String> = pargs.opt_value_from_str("--firmware")?;
    let renderer: Option<String> = pargs.opt_value_from_str("--renderer")?;

    let args = AppArgs {
        sdcard: pargs.opt_value_from_str("--sdcard")?,
        debugger: pargs.contains(["-d", "--debugger"]),
        breakpoint: pargs.opt_value_from_str(["-b", "--breakpoint"])?,
        unlimited_cpu: pargs.contains(["-u", "--unlimited_cpu"]),
        fullscreen: pargs.contains(["-f", "--fullscreen"]),
        zero: pargs.contains(["-z", "--zero"]),
        perfect_scale: pargs.opt_value_from_str("--scale")?,
        mos_bin: pargs.opt_value_from_str("--mos")?,
        vdp_dll: pargs.opt_value_from_str("--vdp")?,
        renderer: if let Some(r) = renderer {
          match r.as_str() {
            "hw" => Renderer::Accelerated,
            "sw" => Renderer::Software,
            _ => {
              println!("Unknown --renderer value: {}. Valid values are: hw, sw", r);
              std::process::exit(0);
            }
          }
        } else {
          Renderer::Accelerated
        },
        firmware: if let Some(ver) = firmware_ver {
          if ver == "1.03" {
            FirmwareVer::quark103
          } else if ver == "quark" {
            FirmwareVer::quark
          } else if ver == "console8" {
            FirmwareVer::console8
          } else if ver == "electron" {
            FirmwareVer::electronhal
          } else {
            println!("Unknown --firmware value: {}. Valid values are: 1.03, quark, console8, electron", ver);
            std::process::exit(0);
          }
        } else {
          FirmwareVer::console8
        }
    };

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}
