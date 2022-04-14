use bevy::prelude::{Component, FromWorld};
use ctru::console::Console;
use ouroboros::self_referencing;

#[self_referencing]
struct InnerGraphics {
    gfx: ctru::Gfx,

    #[borrows(gfx)]
    #[not_covariant]
    consoles: Vec<Console<'this>>,
}

pub struct Graphics(InnerGraphics);

impl Default for Graphics {
    fn default() -> Self {
        let gfx = ctru::Gfx::init().expect("unable to init gfx");
        Self(InnerGraphics::new(gfx, |_| Vec::new()))
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct ConsoleId(usize);

impl Graphics {
    pub fn add_console_with(&mut self, f: impl FnOnce(&ctru::Gfx) -> Console) -> ConsoleId {
        self.0.with_mut(|inner| {
            let id = inner.consoles.len();
            inner.consoles.push(f(inner.gfx));
            ConsoleId(id)
        })
    }

    pub fn gfx(&self) -> &ctru::Gfx {
        self.0.borrow_gfx()
    }

    pub fn with_console(&mut self, id: ConsoleId, f: impl FnOnce(&mut Console)) {
        self.0.with_consoles_mut(|consoles| {
            let console = consoles
                .get_mut(id.0)
                .unwrap_or_else(|| panic!("invalid {id:?}"));
            f(console);
        });
    }
}
