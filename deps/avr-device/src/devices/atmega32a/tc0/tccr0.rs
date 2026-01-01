#[doc = "Register `TCCR0` reader"]
pub struct R(crate::R<TCCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR0` writer"]
pub struct W(crate::W<TCCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TCCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS0` reader - Clock Selects"]
pub type CS0_R = crate::FieldReader<u8, CS0_A>;
#[doc = "Clock Selects\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS0_A {
    #[doc = "0: No Clock Source (Stopped)"]
    VAL_0X00 = 0,
    #[doc = "1: Running, No Prescaling"]
    VAL_0X01 = 1,
    #[doc = "2: Running, CLK/8"]
    VAL_0X02 = 2,
    #[doc = "3: Running, CLK/64"]
    VAL_0X03 = 3,
    #[doc = "4: Running, CLK/256"]
    VAL_0X04 = 4,
    #[doc = "5: Running, CLK/1024"]
    VAL_0X05 = 5,
    #[doc = "6: Running, ExtClk Tx Falling Edge"]
    VAL_0X06 = 6,
    #[doc = "7: Running, ExtClk Tx Rising Edge"]
    VAL_0X07 = 7,
}
impl From<CS0_A> for u8 {
    #[inline(always)]
    fn from(variant: CS0_A) -> Self {
        variant as _
    }
}
impl CS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS0_A {
        match self.bits {
            0 => CS0_A::VAL_0X00,
            1 => CS0_A::VAL_0X01,
            2 => CS0_A::VAL_0X02,
            3 => CS0_A::VAL_0X03,
            4 => CS0_A::VAL_0X04,
            5 => CS0_A::VAL_0X05,
            6 => CS0_A::VAL_0X06,
            7 => CS0_A::VAL_0X07,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VAL_0X00`"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == CS0_A::VAL_0X00
    }
    #[doc = "Checks if the value of the field is `VAL_0X01`"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == CS0_A::VAL_0X01
    }
    #[doc = "Checks if the value of the field is `VAL_0X02`"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == CS0_A::VAL_0X02
    }
    #[doc = "Checks if the value of the field is `VAL_0X03`"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == CS0_A::VAL_0X03
    }
    #[doc = "Checks if the value of the field is `VAL_0X04`"]
    #[inline(always)]
    pub fn is_val_0x04(&self) -> bool {
        *self == CS0_A::VAL_0X04
    }
    #[doc = "Checks if the value of the field is `VAL_0X05`"]
    #[inline(always)]
    pub fn is_val_0x05(&self) -> bool {
        *self == CS0_A::VAL_0X05
    }
    #[doc = "Checks if the value of the field is `VAL_0X06`"]
    #[inline(always)]
    pub fn is_val_0x06(&self) -> bool {
        *self == CS0_A::VAL_0X06
    }
    #[doc = "Checks if the value of the field is `VAL_0X07`"]
    #[inline(always)]
    pub fn is_val_0x07(&self) -> bool {
        *self == CS0_A::VAL_0X07
    }
}
#[doc = "Field `CS0` writer - Clock Selects"]
pub type CS0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR0_SPEC, u8, CS0_A, 3, O>;
impl<'a, const O: u8> CS0_W<'a, O> {
    #[doc = "No Clock Source (Stopped)"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut W {
        self.variant(CS0_A::VAL_0X00)
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut W {
        self.variant(CS0_A::VAL_0X01)
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut W {
        self.variant(CS0_A::VAL_0X02)
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut W {
        self.variant(CS0_A::VAL_0X03)
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn val_0x04(self) -> &'a mut W {
        self.variant(CS0_A::VAL_0X04)
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn val_0x05(self) -> &'a mut W {
        self.variant(CS0_A::VAL_0X05)
    }
    #[doc = "Running, ExtClk Tx Falling Edge"]
    #[inline(always)]
    pub fn val_0x06(self) -> &'a mut W {
        self.variant(CS0_A::VAL_0X06)
    }
    #[doc = "Running, ExtClk Tx Rising Edge"]
    #[inline(always)]
    pub fn val_0x07(self) -> &'a mut W {
        self.variant(CS0_A::VAL_0X07)
    }
}
#[doc = "Field `WGM01` reader - Waveform Generation Mode 1"]
pub type WGM01_R = crate::BitReader<bool>;
#[doc = "Field `WGM01` writer - Waveform Generation Mode 1"]
pub type WGM01_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR0_SPEC, bool, O>;
#[doc = "Field `COM0` reader - Compare Match Output Modes"]
pub type COM0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COM0` writer - Compare Match Output Modes"]
pub type COM0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `WGM00` reader - Waveform Generation Mode"]
pub type WGM00_R = crate::BitReader<WGM00_A>;
#[doc = "Waveform Generation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WGM00_A {
    #[doc = "0: Normal"]
    VAL_0X00 = 0,
    #[doc = "1: CTC"]
    VAL_0X01 = 1,
}
impl From<WGM00_A> for bool {
    #[inline(always)]
    fn from(variant: WGM00_A) -> Self {
        variant as u8 != 0
    }
}
impl WGM00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WGM00_A {
        match self.bits {
            false => WGM00_A::VAL_0X00,
            true => WGM00_A::VAL_0X01,
        }
    }
    #[doc = "Checks if the value of the field is `VAL_0X00`"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == WGM00_A::VAL_0X00
    }
    #[doc = "Checks if the value of the field is `VAL_0X01`"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == WGM00_A::VAL_0X01
    }
}
#[doc = "Field `WGM00` writer - Waveform Generation Mode"]
pub type WGM00_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR0_SPEC, WGM00_A, O>;
impl<'a, const O: u8> WGM00_W<'a, O> {
    #[doc = "Normal"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut W {
        self.variant(WGM00_A::VAL_0X00)
    }
    #[doc = "CTC"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut W {
        self.variant(WGM00_A::VAL_0X01)
    }
}
#[doc = "Field `FOC0` reader - Force Output Compare"]
pub type FOC0_R = crate::BitReader<bool>;
#[doc = "Field `FOC0` writer - Force Output Compare"]
pub type FOC0_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Clock Selects"]
    #[inline(always)]
    pub fn cs0(&self) -> CS0_R {
        CS0_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Waveform Generation Mode 1"]
    #[inline(always)]
    pub fn wgm01(&self) -> WGM01_R {
        WGM01_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Compare Match Output Modes"]
    #[inline(always)]
    pub fn com0(&self) -> COM0_R {
        COM0_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm00(&self) -> WGM00_R {
        WGM00_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Output Compare"]
    #[inline(always)]
    pub fn foc0(&self) -> FOC0_R {
        FOC0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Selects"]
    #[inline(always)]
    #[must_use]
    pub fn cs0(&mut self) -> CS0_W<0> {
        CS0_W::new(self)
    }
    #[doc = "Bit 3 - Waveform Generation Mode 1"]
    #[inline(always)]
    #[must_use]
    pub fn wgm01(&mut self) -> WGM01_W<3> {
        WGM01_W::new(self)
    }
    #[doc = "Bits 4:5 - Compare Match Output Modes"]
    #[inline(always)]
    #[must_use]
    pub fn com0(&mut self) -> COM0_W<4> {
        COM0_W::new(self)
    }
    #[doc = "Bit 6 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm00(&mut self) -> WGM00_W<6> {
        WGM00_W::new(self)
    }
    #[doc = "Bit 7 - Force Output Compare"]
    #[inline(always)]
    #[must_use]
    pub fn foc0(&mut self) -> FOC0_W<7> {
        FOC0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr0](index.html) module"]
pub struct TCCR0_SPEC;
impl crate::RegisterSpec for TCCR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr0::R](R) reader structure"]
impl crate::Readable for TCCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr0::W](W) writer structure"]
impl crate::Writable for TCCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR0 to value 0"]
impl crate::Resettable for TCCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
