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
    #[doc = "0: Ext. Clock; Start-up time: 14 CK + 0 ms"]
    EXTCLK_14CK_0MS = 0,
    #[doc = "1: Int. RC Osc. 4.8 MHz; Start-up time: 14 CK + 0 ms"]
    INTRCOSC_4MHZ8_14CK_0MS = 1,
    #[doc = "2: Int. RC Osc. 9.6 MHz; Start-up time: 14 CK + 0 ms"]
    INTRCOSC_9MHZ6_14CK_0MS = 2,
    #[doc = "3: Int. RC Osc. 128 kHz; Start-up time: 14 CK + 0 ms"]
    INTRCOSC_128KHZ_14CK_0MS = 3,
    #[doc = "4: Ext. Clock; Start-up time: 14 CK + 4 ms"]
    EXTCLK_14CK_4MS = 4,
    #[doc = "5: Int. RC Osc. 4.8 MHz; Start-up time: 14 CK + 4 ms"]
    INTRCOSC_4MHZ8_14CK_4MS = 5,
    #[doc = "6: Int. RC Osc. 9.6 MHz; Start-up time: 14 CK + 4 ms"]
    INTRCOSC_9MHZ6_14CK_4MS = 6,
    #[doc = "7: Int. RC Osc. 128 kHz; Start-up time: 14 CK + 4 ms"]
    INTRCOSC_128KHZ_14CK_4MS = 7,
    #[doc = "8: Ext. Clock; Start-up time: 14 CK + 64 ms"]
    EXTCLK_14CK_64MS = 8,
    #[doc = "9: Int. RC Osc. 4.8 MHz; Start-up time: 14 CK + 64 ms"]
    INTRCOSC_4MHZ8_14CK_64MS = 9,
    #[doc = "10: Int. RC Osc. 9.6 MHz; Start-up time: 14 CK + 64 ms"]
    INTRCOSC_9MHZ6_14CK_64MS = 10,
    #[doc = "11: Int. RC Osc. 128 kHz; Start-up time: 14 CK + 64 ms"]
    INTRCOSC_128KHZ_14CK_64MS = 11,
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
            0 => Some(SUT_CKSEL_A::EXTCLK_14CK_0MS),
            1 => Some(SUT_CKSEL_A::INTRCOSC_4MHZ8_14CK_0MS),
            2 => Some(SUT_CKSEL_A::INTRCOSC_9MHZ6_14CK_0MS),
            3 => Some(SUT_CKSEL_A::INTRCOSC_128KHZ_14CK_0MS),
            4 => Some(SUT_CKSEL_A::EXTCLK_14CK_4MS),
            5 => Some(SUT_CKSEL_A::INTRCOSC_4MHZ8_14CK_4MS),
            6 => Some(SUT_CKSEL_A::INTRCOSC_9MHZ6_14CK_4MS),
            7 => Some(SUT_CKSEL_A::INTRCOSC_128KHZ_14CK_4MS),
            8 => Some(SUT_CKSEL_A::EXTCLK_14CK_64MS),
            9 => Some(SUT_CKSEL_A::INTRCOSC_4MHZ8_14CK_64MS),
            10 => Some(SUT_CKSEL_A::INTRCOSC_9MHZ6_14CK_64MS),
            11 => Some(SUT_CKSEL_A::INTRCOSC_128KHZ_14CK_64MS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EXTCLK_14CK_0MS`"]
    #[inline(always)]
    pub fn is_extclk_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCLK_14CK_0MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_4MHZ8_14CK_0MS`"]
    #[inline(always)]
    pub fn is_intrcosc_4mhz8_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_4MHZ8_14CK_0MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_9MHZ6_14CK_0MS`"]
    #[inline(always)]
    pub fn is_intrcosc_9mhz6_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_9MHZ6_14CK_0MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_128KHZ_14CK_0MS`"]
    #[inline(always)]
    pub fn is_intrcosc_128khz_14ck_0ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_128KHZ_14CK_0MS
    }
    #[doc = "Checks if the value of the field is `EXTCLK_14CK_4MS`"]
    #[inline(always)]
    pub fn is_extclk_14ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCLK_14CK_4MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_4MHZ8_14CK_4MS`"]
    #[inline(always)]
    pub fn is_intrcosc_4mhz8_14ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_4MHZ8_14CK_4MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_9MHZ6_14CK_4MS`"]
    #[inline(always)]
    pub fn is_intrcosc_9mhz6_14ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_9MHZ6_14CK_4MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_128KHZ_14CK_4MS`"]
    #[inline(always)]
    pub fn is_intrcosc_128khz_14ck_4ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_128KHZ_14CK_4MS
    }
    #[doc = "Checks if the value of the field is `EXTCLK_14CK_64MS`"]
    #[inline(always)]
    pub fn is_extclk_14ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::EXTCLK_14CK_64MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_4MHZ8_14CK_64MS`"]
    #[inline(always)]
    pub fn is_intrcosc_4mhz8_14ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_4MHZ8_14CK_64MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_9MHZ6_14CK_64MS`"]
    #[inline(always)]
    pub fn is_intrcosc_9mhz6_14ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_9MHZ6_14CK_64MS
    }
    #[doc = "Checks if the value of the field is `INTRCOSC_128KHZ_14CK_64MS`"]
    #[inline(always)]
    pub fn is_intrcosc_128khz_14ck_64ms(&self) -> bool {
        *self == SUT_CKSEL_A::INTRCOSC_128KHZ_14CK_64MS
    }
}
#[doc = "Field `SUT_CKSEL` writer - Select Clock Source"]
pub type SUT_CKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LOW_SPEC, u8, SUT_CKSEL_A, 4, O>;
impl<'a, const O: u8> SUT_CKSEL_W<'a, O> {
    #[doc = "Ext. Clock; Start-up time: 14 CK + 0 ms"]
    #[inline(always)]
    pub fn extclk_14ck_0ms(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::EXTCLK_14CK_0MS)
    }
    #[doc = "Int. RC Osc. 4.8 MHz; Start-up time: 14 CK + 0 ms"]
    #[inline(always)]
    pub fn intrcosc_4mhz8_14ck_0ms(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::INTRCOSC_4MHZ8_14CK_0MS)
    }
    #[doc = "Int. RC Osc. 9.6 MHz; Start-up time: 14 CK + 0 ms"]
    #[inline(always)]
    pub fn intrcosc_9mhz6_14ck_0ms(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::INTRCOSC_9MHZ6_14CK_0MS)
    }
    #[doc = "Int. RC Osc. 128 kHz; Start-up time: 14 CK + 0 ms"]
    #[inline(always)]
    pub fn intrcosc_128khz_14ck_0ms(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::INTRCOSC_128KHZ_14CK_0MS)
    }
    #[doc = "Ext. Clock; Start-up time: 14 CK + 4 ms"]
    #[inline(always)]
    pub fn extclk_14ck_4ms(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::EXTCLK_14CK_4MS)
    }
    #[doc = "Int. RC Osc. 4.8 MHz; Start-up time: 14 CK + 4 ms"]
    #[inline(always)]
    pub fn intrcosc_4mhz8_14ck_4ms(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::INTRCOSC_4MHZ8_14CK_4MS)
    }
    #[doc = "Int. RC Osc. 9.6 MHz; Start-up time: 14 CK + 4 ms"]
    #[inline(always)]
    pub fn intrcosc_9mhz6_14ck_4ms(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::INTRCOSC_9MHZ6_14CK_4MS)
    }
    #[doc = "Int. RC Osc. 128 kHz; Start-up time: 14 CK + 4 ms"]
    #[inline(always)]
    pub fn intrcosc_128khz_14ck_4ms(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::INTRCOSC_128KHZ_14CK_4MS)
    }
    #[doc = "Ext. Clock; Start-up time: 14 CK + 64 ms"]
    #[inline(always)]
    pub fn extclk_14ck_64ms(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::EXTCLK_14CK_64MS)
    }
    #[doc = "Int. RC Osc. 4.8 MHz; Start-up time: 14 CK + 64 ms"]
    #[inline(always)]
    pub fn intrcosc_4mhz8_14ck_64ms(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::INTRCOSC_4MHZ8_14CK_64MS)
    }
    #[doc = "Int. RC Osc. 9.6 MHz; Start-up time: 14 CK + 64 ms"]
    #[inline(always)]
    pub fn intrcosc_9mhz6_14ck_64ms(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::INTRCOSC_9MHZ6_14CK_64MS)
    }
    #[doc = "Int. RC Osc. 128 kHz; Start-up time: 14 CK + 64 ms"]
    #[inline(always)]
    pub fn intrcosc_128khz_14ck_64ms(self) -> &'a mut W {
        self.variant(SUT_CKSEL_A::INTRCOSC_128KHZ_14CK_64MS)
    }
}
#[doc = "Field `CKDIV8` reader - Divide clock by 8 internally"]
pub type CKDIV8_R = crate::BitReader<bool>;
#[doc = "Field `CKDIV8` writer - Divide clock by 8 internally"]
pub type CKDIV8_W<'a, const O: u8> = crate::BitWriter<'a, u8, LOW_SPEC, bool, O>;
#[doc = "Field `WDTON` reader - Watch-dog Timer always on"]
pub type WDTON_R = crate::BitReader<bool>;
#[doc = "Field `WDTON` writer - Watch-dog Timer always on"]
pub type WDTON_W<'a, const O: u8> = crate::BitWriter<'a, u8, LOW_SPEC, bool, O>;
#[doc = "Field `EESAVE` reader - Preserve EEPROM through the Chip Erase cycle"]
pub type EESAVE_R = crate::BitReader<bool>;
#[doc = "Field `EESAVE` writer - Preserve EEPROM through the Chip Erase cycle"]
pub type EESAVE_W<'a, const O: u8> = crate::BitWriter<'a, u8, LOW_SPEC, bool, O>;
#[doc = "Field `SPIEN` reader - Serial program downloading (SPI) enabled"]
pub type SPIEN_R = crate::BitReader<bool>;
#[doc = "Field `SPIEN` writer - Serial program downloading (SPI) enabled"]
pub type SPIEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, LOW_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Select Clock Source"]
    #[inline(always)]
    pub fn sut_cksel(&self) -> SUT_CKSEL_R {
        SUT_CKSEL_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Divide clock by 8 internally"]
    #[inline(always)]
    pub fn ckdiv8(&self) -> CKDIV8_R {
        CKDIV8_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Watch-dog Timer always on"]
    #[inline(always)]
    pub fn wdton(&self) -> WDTON_R {
        WDTON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Preserve EEPROM through the Chip Erase cycle"]
    #[inline(always)]
    pub fn eesave(&self) -> EESAVE_R {
        EESAVE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Serial program downloading (SPI) enabled"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select Clock Source"]
    #[inline(always)]
    #[must_use]
    pub fn sut_cksel(&mut self) -> SUT_CKSEL_W<0> {
        SUT_CKSEL_W::new(self)
    }
    #[doc = "Bit 4 - Divide clock by 8 internally"]
    #[inline(always)]
    #[must_use]
    pub fn ckdiv8(&mut self) -> CKDIV8_W<4> {
        CKDIV8_W::new(self)
    }
    #[doc = "Bit 5 - Watch-dog Timer always on"]
    #[inline(always)]
    #[must_use]
    pub fn wdton(&mut self) -> WDTON_W<5> {
        WDTON_W::new(self)
    }
    #[doc = "Bit 6 - Preserve EEPROM through the Chip Erase cycle"]
    #[inline(always)]
    #[must_use]
    pub fn eesave(&mut self) -> EESAVE_W<6> {
        EESAVE_W::new(self)
    }
    #[doc = "Bit 7 - Serial program downloading (SPI) enabled"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SPIEN_W<7> {
        SPIEN_W::new(self)
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
