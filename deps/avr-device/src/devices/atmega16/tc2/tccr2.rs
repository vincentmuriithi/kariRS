#[doc = "Register `TCCR2` reader"]
pub struct R(crate::R<TCCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR2` writer"]
pub struct W(crate::W<TCCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR2_SPEC>;
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
impl From<crate::W<TCCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS2` reader - Clock Select bits"]
pub type CS2_R = crate::FieldReader<u8, CS2_A>;
#[doc = "Clock Select bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS2_A {
    #[doc = "0: No Clock Source (Stopped)"]
    VAL_0X00 = 0,
    #[doc = "1: Running, No Prescaling"]
    VAL_0X01 = 1,
    #[doc = "2: Running, CLK/8"]
    VAL_0X02 = 2,
    #[doc = "3: Running, CLK/32"]
    VAL_0X03 = 3,
    #[doc = "4: Running, CLK/64"]
    VAL_0X04 = 4,
    #[doc = "5: Running, CLK/128"]
    VAL_0X05 = 5,
    #[doc = "6: Running, CLK/256"]
    VAL_0X06 = 6,
    #[doc = "7: Running, CLK/1024"]
    VAL_0X07 = 7,
}
impl From<CS2_A> for u8 {
    #[inline(always)]
    fn from(variant: CS2_A) -> Self {
        variant as _
    }
}
impl CS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS2_A {
        match self.bits {
            0 => CS2_A::VAL_0X00,
            1 => CS2_A::VAL_0X01,
            2 => CS2_A::VAL_0X02,
            3 => CS2_A::VAL_0X03,
            4 => CS2_A::VAL_0X04,
            5 => CS2_A::VAL_0X05,
            6 => CS2_A::VAL_0X06,
            7 => CS2_A::VAL_0X07,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VAL_0X00`"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == CS2_A::VAL_0X00
    }
    #[doc = "Checks if the value of the field is `VAL_0X01`"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == CS2_A::VAL_0X01
    }
    #[doc = "Checks if the value of the field is `VAL_0X02`"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == CS2_A::VAL_0X02
    }
    #[doc = "Checks if the value of the field is `VAL_0X03`"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == CS2_A::VAL_0X03
    }
    #[doc = "Checks if the value of the field is `VAL_0X04`"]
    #[inline(always)]
    pub fn is_val_0x04(&self) -> bool {
        *self == CS2_A::VAL_0X04
    }
    #[doc = "Checks if the value of the field is `VAL_0X05`"]
    #[inline(always)]
    pub fn is_val_0x05(&self) -> bool {
        *self == CS2_A::VAL_0X05
    }
    #[doc = "Checks if the value of the field is `VAL_0X06`"]
    #[inline(always)]
    pub fn is_val_0x06(&self) -> bool {
        *self == CS2_A::VAL_0X06
    }
    #[doc = "Checks if the value of the field is `VAL_0X07`"]
    #[inline(always)]
    pub fn is_val_0x07(&self) -> bool {
        *self == CS2_A::VAL_0X07
    }
}
#[doc = "Field `CS2` writer - Clock Select bits"]
pub type CS2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR2_SPEC, u8, CS2_A, 3, O>;
impl<'a, const O: u8> CS2_W<'a, O> {
    #[doc = "No Clock Source (Stopped)"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut W {
        self.variant(CS2_A::VAL_0X00)
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut W {
        self.variant(CS2_A::VAL_0X01)
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut W {
        self.variant(CS2_A::VAL_0X02)
    }
    #[doc = "Running, CLK/32"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut W {
        self.variant(CS2_A::VAL_0X03)
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn val_0x04(self) -> &'a mut W {
        self.variant(CS2_A::VAL_0X04)
    }
    #[doc = "Running, CLK/128"]
    #[inline(always)]
    pub fn val_0x05(self) -> &'a mut W {
        self.variant(CS2_A::VAL_0X05)
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn val_0x06(self) -> &'a mut W {
        self.variant(CS2_A::VAL_0X06)
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn val_0x07(self) -> &'a mut W {
        self.variant(CS2_A::VAL_0X07)
    }
}
#[doc = "Field `CS` reader - Clock Select"]
pub type CS_R = crate::FieldReader<u8, CS_A>;
#[doc = "Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS_A {
    #[doc = "0: No clock source (Timer/Counter stopped)"]
    NO_CLOCK = 0,
    #[doc = "1: Running, No Prescaling"]
    DIRECT = 1,
    #[doc = "2: Running, CLK/8"]
    PRESCALE_8 = 2,
    #[doc = "3: Running, CLK/32"]
    PRESCALE_32 = 3,
    #[doc = "4: Running, CLK/64"]
    PRESCALE_64 = 4,
    #[doc = "5: Running, CLK/128"]
    PRESCALE_128 = 5,
    #[doc = "6: Running, CLK/256"]
    PRESCALE_256 = 6,
    #[doc = "7: Running, CLK/1024"]
    PRESCALE_1024 = 7,
}
impl From<CS_A> for u8 {
    #[inline(always)]
    fn from(variant: CS_A) -> Self {
        variant as _
    }
}
impl CS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS_A {
        match self.bits {
            0 => CS_A::NO_CLOCK,
            1 => CS_A::DIRECT,
            2 => CS_A::PRESCALE_8,
            3 => CS_A::PRESCALE_32,
            4 => CS_A::PRESCALE_64,
            5 => CS_A::PRESCALE_128,
            6 => CS_A::PRESCALE_256,
            7 => CS_A::PRESCALE_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLOCK`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == CS_A::NO_CLOCK
    }
    #[doc = "Checks if the value of the field is `DIRECT`"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == CS_A::DIRECT
    }
    #[doc = "Checks if the value of the field is `PRESCALE_8`"]
    #[inline(always)]
    pub fn is_prescale_8(&self) -> bool {
        *self == CS_A::PRESCALE_8
    }
    #[doc = "Checks if the value of the field is `PRESCALE_32`"]
    #[inline(always)]
    pub fn is_prescale_32(&self) -> bool {
        *self == CS_A::PRESCALE_32
    }
    #[doc = "Checks if the value of the field is `PRESCALE_64`"]
    #[inline(always)]
    pub fn is_prescale_64(&self) -> bool {
        *self == CS_A::PRESCALE_64
    }
    #[doc = "Checks if the value of the field is `PRESCALE_128`"]
    #[inline(always)]
    pub fn is_prescale_128(&self) -> bool {
        *self == CS_A::PRESCALE_128
    }
    #[doc = "Checks if the value of the field is `PRESCALE_256`"]
    #[inline(always)]
    pub fn is_prescale_256(&self) -> bool {
        *self == CS_A::PRESCALE_256
    }
    #[doc = "Checks if the value of the field is `PRESCALE_1024`"]
    #[inline(always)]
    pub fn is_prescale_1024(&self) -> bool {
        *self == CS_A::PRESCALE_1024
    }
}
#[doc = "Field `CS` writer - Clock Select"]
pub type CS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR2_SPEC, u8, CS_A, 3, O>;
impl<'a, const O: u8> CS_W<'a, O> {
    #[doc = "No clock source (Timer/Counter stopped)"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(CS_A::NO_CLOCK)
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut W {
        self.variant(CS_A::DIRECT)
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn prescale_8(self) -> &'a mut W {
        self.variant(CS_A::PRESCALE_8)
    }
    #[doc = "Running, CLK/32"]
    #[inline(always)]
    pub fn prescale_32(self) -> &'a mut W {
        self.variant(CS_A::PRESCALE_32)
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn prescale_64(self) -> &'a mut W {
        self.variant(CS_A::PRESCALE_64)
    }
    #[doc = "Running, CLK/128"]
    #[inline(always)]
    pub fn prescale_128(self) -> &'a mut W {
        self.variant(CS_A::PRESCALE_128)
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn prescale_256(self) -> &'a mut W {
        self.variant(CS_A::PRESCALE_256)
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn prescale_1024(self) -> &'a mut W {
        self.variant(CS_A::PRESCALE_1024)
    }
}
#[doc = "Field `WGM21` reader - Waveform Generation Mode"]
pub type WGM21_R = crate::BitReader<bool>;
#[doc = "Field `WGM21` writer - Waveform Generation Mode"]
pub type WGM21_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR2_SPEC, bool, O>;
#[doc = "Field `COM2` reader - Compare Output Mode bits"]
pub type COM2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COM2` writer - Compare Output Mode bits"]
pub type COM2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `WGM20` reader - Waveform Genration Mode"]
pub type WGM20_R = crate::BitReader<WGM20_A>;
#[doc = "Waveform Genration Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WGM20_A {
    #[doc = "0: Normal"]
    VAL_0X00 = 0,
    #[doc = "1: CTC"]
    VAL_0X01 = 1,
}
impl From<WGM20_A> for bool {
    #[inline(always)]
    fn from(variant: WGM20_A) -> Self {
        variant as u8 != 0
    }
}
impl WGM20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WGM20_A {
        match self.bits {
            false => WGM20_A::VAL_0X00,
            true => WGM20_A::VAL_0X01,
        }
    }
    #[doc = "Checks if the value of the field is `VAL_0X00`"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == WGM20_A::VAL_0X00
    }
    #[doc = "Checks if the value of the field is `VAL_0X01`"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == WGM20_A::VAL_0X01
    }
}
#[doc = "Field `WGM20` writer - Waveform Genration Mode"]
pub type WGM20_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR2_SPEC, WGM20_A, O>;
impl<'a, const O: u8> WGM20_W<'a, O> {
    #[doc = "Normal"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut W {
        self.variant(WGM20_A::VAL_0X00)
    }
    #[doc = "CTC"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut W {
        self.variant(WGM20_A::VAL_0X01)
    }
}
#[doc = "Field `FOC2` reader - Force Output Compare"]
pub type FOC2_R = crate::BitReader<bool>;
#[doc = "Field `FOC2` writer - Force Output Compare"]
pub type FOC2_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Clock Select bits"]
    #[inline(always)]
    pub fn cs2(&self) -> CS2_R {
        CS2_R::new(self.bits & 7)
    }
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm21(&self) -> WGM21_R {
        WGM21_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Compare Output Mode bits"]
    #[inline(always)]
    pub fn com2(&self) -> COM2_R {
        COM2_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Waveform Genration Mode"]
    #[inline(always)]
    pub fn wgm20(&self) -> WGM20_R {
        WGM20_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Output Compare"]
    #[inline(always)]
    pub fn foc2(&self) -> FOC2_R {
        FOC2_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn cs2(&mut self) -> CS2_W<0> {
        CS2_W::new(self)
    }
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<0> {
        CS_W::new(self)
    }
    #[doc = "Bit 3 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm21(&mut self) -> WGM21_W<3> {
        WGM21_W::new(self)
    }
    #[doc = "Bits 4:5 - Compare Output Mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn com2(&mut self) -> COM2_W<4> {
        COM2_W::new(self)
    }
    #[doc = "Bit 6 - Waveform Genration Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm20(&mut self) -> WGM20_W<6> {
        WGM20_W::new(self)
    }
    #[doc = "Bit 7 - Force Output Compare"]
    #[inline(always)]
    #[must_use]
    pub fn foc2(&mut self) -> FOC2_W<7> {
        FOC2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter2 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr2](index.html) module"]
pub struct TCCR2_SPEC;
impl crate::RegisterSpec for TCCR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr2::R](R) reader structure"]
impl crate::Readable for TCCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr2::W](W) writer structure"]
impl crate::Writable for TCCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR2 to value 0"]
impl crate::Resettable for TCCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
