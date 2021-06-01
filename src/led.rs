use crate::hal::gpio::{
    Pin, Floating, Output,
    porte::PE26,
    portb::{PB21, PB22}
};

pub enum LedColor {
    Green,
    Blue,
    Red,
}

pub type LD0 = PE26<Output<Floating>>;
pub type LD1 = PB21<Output<Floating>>;
pub type LD2 = PB22<Output<Floating>>;

pub struct LedController {
    leds: [Led; 3],
}

impl LedController {
    pub fn new(green: LD0,
               blue: LD1,
               red: LD2) -> LedController {
        LedController { leds: [green.into(), blue.into(), red.into()] }
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<Led> {
        self.leds.iter_mut()
    }
}

impl core::ops::Deref for LedController {
    type Target = [Led];

    fn deref(&self) -> &Self::Target {
        &self.leds
    }
}

impl core::ops::DerefMut for LedController {
    fn deref_mut(&mut self) -> &mut [Led] {
        &mut self.leds
    }
}

impl core::ops::Index<usize> for LedController {
    type Output = Led;

    fn index(&self, i: usize) -> &Self::Output {
        &self.leds[i]
    }
}

impl core::ops::Index<LedColor> for LedController {
    type Output = Led;

    fn index(&self, color: LedColor) -> &Self::Output {
        &self.leds[color as usize]
    }
}

impl core::ops::IndexMut<usize> for LedController {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.leds[i]
    }
}

impl core::ops::IndexMut<LedColor> for LedController {
    fn index_mut(&mut self, color: LedColor) -> &mut Self::Output {
        &mut self.leds[color as usize]
    }
}

pub struct Led {
    pin: Pin<Output<Floating>>
}

macro_rules! ctor {
    ($($ldx:ident),+) => {
        $(
            impl Into<Led> for $ldx {
                fn into(self) -> Led {
                    Led {
                        pin: self.donwgrade(),
                    }
                }
            }
        )+
    }
}

ctor!(LD0, LD1, LD2);

impl Led {
    pub fn off(&mut self) {
        self.pin.set_low().ok();
    }

    pub fn on(&mut self) {
        self.pin.set_high().ok();
    }

    pub fn toggle(&mut self) {
        if let Ok(true) = self.pin.is_set_low() {
            self.pin.set_high().ok();
        } else {
            self.pin.set_low().ok();
        }
    }
}
