use eframe::egui::{
	Context,
	CentralPanel,
	Slider, SidePanel, TopBottomPanel,
    Layout, Align, Window, 
};
use eframe::epaint::Vec2;
use eframe::{Frame,App};
struct MyApp {
    data: Vec<Vec<u8>>,
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            data: vec![vec![0;9]; 9],
        }
    }
}
impl App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Input");
                ui.label("Input the sudoku puzzle here");
                ui.horizontal(|ui| {
                    for i in 0..9 {
                        ui.vertical(|ui| {
                            for j in 0..9 {
                                let mut input_text = self.data[i][j].to_string();
                                ui.add(eframe::egui::TextEdit::singleline(&mut input_text).margin(Vec2::new(2., 2.)).desired_width(20.0));
                                self.data[i][j] = input_text.parse().unwrap_or(0);
                            }
                        });
                    }
                });
                ui.separator();
                ui.horizontal(|ui|{
                    if ui.button("Solve").clicked() {
                        let mut s = Sudoku::new(self.data.clone());
                        s.solve(0);
                        self.data = s.data;
                    }
                    if ui.button("reset").clicked() {
                        self.data = vec![vec![0;9]; 9];
                    }
                });
            });
        });
    }
}
struct Sudoku {
    data: Vec<Vec<u8>>,
}
impl Sudoku {
    fn new(data: Vec<Vec<u8>>) -> Self {
        Self { data }
    }
    fn is_valid(&self) -> bool {
        let mut rows = vec![0; 9];
        let mut cols = vec![0; 9];
        let mut blocks = vec![0; 9];
        for i in 0..9 {
            for j in 0..9 {
                let n = self.data[i][j];
                if n == 0 {
                    continue;
                }
                let n = 1 << (n - 1);
                if rows[i] & n != 0 {
                    return false;
                }
                rows[i] |= n;
                if cols[j] & n != 0 {
                    return false;
                }
                cols[j] |= n;
                let k = i / 3 * 3 + j / 3;
                if blocks[k] & n != 0 {
                    return false;
                }
                blocks[k] |= n;
            }
        }
        true
    }
    fn solve(&mut self, n:usize) -> bool {
        if n == 81 {
            return true;
        } else {
            let x = n/9;
            let y = n%9;
            if self.data[x][y] != 0 {
                return self.solve(n+1);
            } else {
                for i in 1..10 {
                    self.data[x][y] = i;
                    if self.is_valid() && self.solve(n+1) {
                        return true;
                    }
                }
            }
            self.data[x][y] = 0;
        }
        false
    }
}
fn main() {
    let _ = eframe::run_native("Sudoku Solver", eframe::NativeOptions::default(), Box::new(|_| Box::new(MyApp::default())));
}