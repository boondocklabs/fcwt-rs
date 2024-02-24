use egui::{epaint::Hsva, load::SizedTexture, CentralPanel, Color32, ColorImage, Context, Image, ImageSource, RichText, SidePanel, TextureHandle};
use fcwt::{scales::LinFreqs, wavelet::Wavelet, CwtResult, FastCwt, MorletWavelet};
use egui_plot::{Line, Plot, PlotPoints};
//use csv::Writer;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {

    #[cfg(feature="profile")]
    puffin::set_scopes_on(true);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([2000.0, 1080.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "fCWT Image Display",
        options,
        Box::new(|_cc| Box::new(WaveletDemo::default())),
    ).unwrap();

}

struct WaveletDemo {
    texture: Option<TextureHandle>,
    image: Option<ColorImage>,

    fs: usize,
    sigma: f32,
    size: usize,
    scale: f32,

    f0: f32,
    f1: f32,

    signal: Vec<f32>,
    signal_size: usize,

    normalize: bool,

    fcwt: FastCwt<MorletWavelet, LinFreqs>,
    output: Option<CwtResult<f32>>,
}

impl Default for WaveletDemo {
    fn default() -> Self {
        let sigma = 2.0;
        let fs: usize = 2000;
        let size: usize = 300;
        let scale: f32 = 100.0;

        let f0 = 1f32;
        let f1 = 50.0f32;

        let wavelet = MorletWavelet::new(sigma);
        let scales = LinFreqs::new(&wavelet, fs, f0, f1*2.0, size);

        let signal_size = 8192u32.next_power_of_two() as usize;

        let signal = fcwt::util::chirp(fs as f32, signal_size, f0, f1);

        let normalize = true;

        let fcwt = FastCwt::new(wavelet, scales, normalize);

        Self {
            texture: None,
            image: None,
            fs,
            sigma,
            size,
            f0,
            f1,
            signal,
            fcwt,
            scale,
            signal_size,
            output: None,
            normalize,
        }
    }
}

/*
fn save_csv(filename: String, data: &CwtResult<f32>) {
    let mut writer = Writer::from_path(filename).unwrap();
    for row in data.rows().iter() {
        let r: Vec<f32> = row.iter().map(|x| x.norm()).collect();
        writer.serialize(r).unwrap();
    }
    writer.flush().unwrap();
}

fn save_signal_csv(filename: String, data: &Vec<f32>) {
    let mut writer = Writer::from_path(filename).unwrap();
    writer.serialize(data).unwrap();
    writer.flush().unwrap();
}
*/

impl eframe::App for WaveletDemo {


    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {

        //ctx.set_pixels_per_point(2.0);

        puffin_egui::profiler_window(ctx);

        if self.texture.is_none() & !self.image.is_none(){
            // Allocate a new texture
            if let Some(image) = &self.image {
                let texture = ctx.load_texture("cwt", image.clone(), Default::default());
                self.texture = Some(texture);
            }
        }

        SidePanel::left("Left").show(ctx, |ui| {
            ui.label(RichText::new("Common").color(Color32::LIGHT_BLUE));
            ui.add(egui::Slider::new(&mut self.fs, 1000..=120000).text("Sample Rate"));
            ui.separator();

            ui.label(RichText::new("Wavelet").color(Color32::LIGHT_BLUE));
            ui.add(egui::Slider::new(&mut self.sigma, 1.0..=16.0).text("Wavelet Sigma"));
            ui.add(egui::Slider::new(&mut self.size, 1..=1000).text("Size"));
            ui.add(egui::Slider::new(&mut self.scale, 1.0..=1000.0).text("Scale"));
            ui.separator();

            ui.label(RichText::new("Chirp Signal").color(Color32::LIGHT_BLUE));
            ui.add(egui::Slider::new(&mut self.f0, 1.0..=100.0).text("Start Freq"));
            ui.add(egui::Slider::new(&mut self.f1, self.f0..=(self.fs as f32 / 4.0)).text("End Freq"));

            ui.separator();
            if ui.button("Update Transform").clicked() {
                let output = self.fcwt.cwt(&self.signal);
                self.output = Some(output);
                self.update_image();

                // Save to transform.csv
                //save_csv("transform.csv".to_string(), &output);
                //save_signal_csv("signal.csv".to_string(), &self.signal);
            };

        });

        CentralPanel::default().show(ctx, |ui| {

            /*
            let mother_points: PlotPoints = self.fcwt.wavelet().mother().iter().enumerate().map(|(x,&v)| {
                [x as f64 * 0.01, v as f64]
            }).collect();
            let mother_line = Line::new(mother_points);
            */

            let wave = self.fcwt.wavelet().generate(self.size, self.scale);
            let wave_points: PlotPoints = wave.iter().enumerate().map(|(x,&v)| {
                [x as f64, v.re as f64]
            }).collect();

            let signal_points: PlotPoints = self.signal.iter().enumerate().map(|(x,&v)| {
                [x as f64, v as f64]
            }).collect();
            let signal_line = Line::new(signal_points);

            let wave_line = Line::new(wave_points);

            Plot::new("wavelet_plot")
                .view_aspect(3.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(wave_line);
                });

            Plot::new("signal_plot")
                .view_aspect(3.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(signal_line);
                });

            if let Some(handle) = &self.texture {
                let texture = SizedTexture::from_handle(handle);
                let image = Image::new(ImageSource::Texture(texture))
                    .shrink_to_fit();
                ui.add(image);
            }

        });

        if self.f1 <= self.fs as f32 / 2.0 {
            let wavelet = MorletWavelet::new(self.sigma);
            let scales = LinFreqs::new(&wavelet, self.fs, self.f0, self.f1*2.0, self.size);
            self.fcwt = FastCwt::new(wavelet, scales, self.normalize);

            self.signal = fcwt::util::chirp(self.fs as f32, self.signal_size, self.f0, self.f1);
        }


    }
}

impl WaveletDemo {
    fn update_image(&mut self) {

        // Get pixel value from the fCWT result
        if let Some(output) = &self.output {

            if self.image.is_none() || self.image.as_ref().unwrap().height() != output.num_scales() {
                self.image = Some(egui::ColorImage::new([output[0].len(),output.num_scales()], Color32::LIGHT_YELLOW));
            }

            for y in 0..output.num_scales() {
                for x in 0..output[0].len() {
                    let val = output[y][x];
                    let c = Hsva::new(val.norm(), 1.0, 1.0, 1.0);
                    if let Some(image) = &mut self.image {
                        image.pixels[y * output[0].len() + x] = c.into();
                    }
                }
            }

            if let Some(handle) = &mut self.texture {
                if let Some(img) = &self.image {
                    handle.set(img.clone(), Default::default());
                }
            }
        }
    }
}