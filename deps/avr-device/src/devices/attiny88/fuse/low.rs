#[doc = "Register `LOW` reader"]
pub struct R(crate::R<LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOW` writer"]
pub struct W(crate::W<LOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOW_SPEC>;
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
impl From<crate::W<LOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUT_CKSEL` reader - Select Clock Source"]
pub type SUT_CKSEL_R = crate::FieldReader<u8, SUT_CKSEL_A>;
#[doc = "Select Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SUT_CKSEL_A {
    #[doc = "12: Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms"]
    EXTCLK_6CK_14CK_0MS = 12,
    #[doc = "14: Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms"]
    INTRCOSC_8MHZ_6CK_14CK_0MS = 14,
    #[doc = "15: Int. RC Osc. 128kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms"]
    INTRCOSC_128KHZ_6CK_14CK_0MS = 15,
    #[doc = "28: Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4.1 ms"]
    EXTCLK_6CK_14CK_4MS1 = 28,
    #[doc = "30: Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4.1 ms"]
    INTRCOSC_8MHZ_6CK_14CK_4MS1 = 30,
    #[doc = "31: Int. RC Osc. 128kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4.1 ms"]
    INTRCOSC_128KHZ_6CK_14CK_4MS1 = 31,
    #[doc = "44: Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 65 ms"]
    EXTCLK_6CK_14CK_65MS = 44,
    #[doc = "46: Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 65 ms; default value"]
    INTRCOSC_8MHZ_6CK_14CK_65MS_DEFAULT = 46,
    #[doc = "47: Int. RC Osc. 128kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 65 ms"]
    INTRCOSC_128KHZ_6CK_14CK_65MS = 47,
}
impl From<SUT_CKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SUT_CKSEL_A) -> Self {
        variant as _
    }
}
impl SUT_CKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SUT_CKSEL_A> {
        match self.bits {
            12 => Some(SUT_CKSEL_A::EXTCLK_6CK_14CK_0MS),
            14 => Some(SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_14CK_0MS),
            15 => Some(SUT_CKSEL_A::INTRCOSC_128KHZ_6CK_14CK_0MS),
            28 => Some(SUT_CKSEL_A::EXTCLK_6CK_14CK_4MS1),
            30 => Some(SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_14CK_4MS1),
            31 => Some(SUT_CKSEL_A::INTRCOSC_128KHZ_6CK_14CK_4MS1),
            44 => Some(SUT_CKSEL_A::EXTCLK_6CK_14CK_65MS),
            46 => Some(SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_14CK_65MS_DEFAULT),
            47 => Some(SUT_CKSEL_A::INTRCOSC_128KHZ_6CK_14CK_65MS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EXTCLK_6CK_14CK_0MS`"]
    #[inline(always)]
    pub fn is_extclk_6ck_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCLK_6CK_14CK_0MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_8MHZ_6CK_14CK_0MS`"]
    #[inline(always)]
    pub fn is_intrcosc_8mhz_6ck_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_14CK_0MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_128KHZ_6CK_14CK_0MS`"]
    #[inline(always)]
    pub fn is_intrcosc_128khz_6ck_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_128KHZ_6CK_14CK_0MS
    }
    #[doc = "Checks if the value of the field is `EXTCLK_6CK_14CK_4MS1`"]
    #[inline(always)]
    pub fn is_extclk_6ck_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCLK_6CK_14CK_4MS1
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_8MHZ_6CK_14CK_4MS1`"]
    #[inline(always)]
    pub fn is_intrcosc_8mhz_6ck_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_14CK_4MS1
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_128KHZ_6CK_14CK_4MS1`"]
    #[inline(always)]
    pub fn is_intrcosc_128khz_6ck_14ck_4ms1(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_128KHZ_6CK_14CK_4MS1
    }
    #[doc = "Checks if the value of the field is `EXTCLK_6CK_14CK_65MS`"]
    #[inline(always)]
    pub fn is_extclk_6ck_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCLK_6CK_14CK_65MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_8MHZ_6CK_14CK_65MS_DEFAULT`"]
    #[inline(always)]
    pub fn is_intrcosc_8mhz_6ck_14ck_65ms_default(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_14CK_65MS_DEFAULT
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_128KHZ_6CK_14CK_65MS`"]
    #[inline(always)]
    pub fn is_intrcosc_128khz_6ck_14ck_65ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_128KHZ_6CK_14CK_65MS
    }
}
#[doc = "Field `SUT_CKSEL` writer - Select Clock Source"]
pub type SUT_CKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LOW_SPEC, u8, SUT_CKSEL_A, 6, O>;
impl<'a, const O: u8> SUT_CKSEL_W<'a, O> {
    #[doc = "Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms"]
    #[inline(always)]
    pub fn extclk_6ck_14ck_0ms(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::EXTCLK_6CK_14CK_0MS)
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms"]
    #[inline(always)]
    pub fn intrcosc_8mhz_6ck_14ck_0ms(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_14CK_0MS)
    }
    #[doc = "Int. RC Osc. 128kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms"]
    #[inline(always)]
    pub fn intrcosc_128khz_6ck_14ck_0ms(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::INTRCOSC_128KHZ_6CK_14CK_0MS)
    }
    #[doc = "Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn extclk_6ck_14ck_4ms1(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::EXTCLK_6CK_14CK_4MS1)
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn intrcosc_8mhz_6ck_14ck_4ms1(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_14CK_4MS1)
    }
    #[doc = "Int. RC Osc. 128kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4.1 ms"]
    #[inline(always)]
    pub fn intrcosc_128khz_6ck_14ck_4ms1(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::INTRCOSC_128KHZ_6CK_14CK_4MS1)
    }
    #[doc = "Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 65 ms"]
    #[inline(always)]
    pub fn extclk_6ck_14ck_65ms(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::EXTCLK_6CK_14CK_65MS)
    }
    #[doc = "Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 65 ms; default value"]
    #[inline(always)]
    pub fn intrcosc_8mhz_6ck_14ck_65ms_default(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::INTRCOSC_8MHZ_6CK_14CK_65MS_DEFAULT)
    }
    #[doc = "Int. RC Osc. 128kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 65 ms"]
    #[inline(always)]
    pub fn intrcosc_128khz_6ck_14ck_65ms(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::INTRCOSC_128KHZ_6CK_14CK_65MS)
    }
}
#[doc = "Field `CKOUT` reader - Clock output on PORTB0"]
pub type CKOUT_R = crate::BitReader<bool>;
#[doc = "Field `CKOUT` writer - Clock output on PORTB0"]
pub type CKOUT_W<'a, const O: u8> = crate::BitWriter<'a, u8, LOW_SPEC, bool, O>;
#[doc = "Field `CKDIV8` reader - Divide clock by 8 internally"]
pub type CKDIV8_R = crate::BitReader<bool>;
#[doc = "Field `CKDIV8` writer - Divide clock by 8 internally"]
pub type CKDIV8_W<'a, const O: u8> = crate::BitWriter<'a, u8, LOW_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Select Clock Source"]
    #[inline(always)]
    pub fn sut_cksel(&self) -> SUT_CKSEL_R {
        SUT_CKSEL_R::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - Clock output on PORTB0"]
    #[inline(always)]
    pub fn ckout(&self) -> CKOUT_R {
        CKOUT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Divide clock by 8 internally"]
    #[inline(always)]
    pub fn ckdiv8(&self) -> CKDIV8_R {
        CKDIV8_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Select Clock Source"]
    #[inline(always)]
    #[must_use]
    pub fn sut_cksel(&mut self) -> SUT_CKSEL_W<0> {
        SUT_CKSEL_W::new(self)
    }
    #[doc = "Bit 6 - Clock output on PORTB0"]
    #[inline(always)]
    #[must_use]
    pub fn ckout(&mut self) -> CKOUT_W<6> {
        CKOUT_W::new(self)
    }
    #[doc = "Bit 7 - Divide clock by 8 internally"]
    #[inline(always)]
    #[must_use]
    pub fn ckdiv8(&mut self) -> CKDIV8_W<7> {
        CKDIV8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [low](index.html) module"]
pub struct LOW_SPEC;
impl crate::RegisterSpec for LOW_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [low::R](R) reader structure"]
impl crate::Readable for LOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [low::W](W) writer structure"]
impl crate::Writable for LOW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOW to value 0"]
impl crate::Resettable for LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
