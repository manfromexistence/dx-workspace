use fb_term::Term;

pub struct Panic;

impl Panic {
	pub fn install() {
		better_panic::install();

		let hook = std::panic::take_hook();
		std::panic::set_hook(Box::new(move |info| {
			Term::goodbye(|| {
				hook(info);
				1
			});
		}));
	}
}
