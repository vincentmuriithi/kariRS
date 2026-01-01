#[doc = "Register `SYSCFG0` reader"]
pub struct R(crate::R<SYSCFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG0` writer"]
pub struct W(crate::W<SYSCFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG0_SPEC>;
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
impl From<crate::W<SYSCFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EESAVE` reader - EEPROM Save"]
pub type EESAVE_R = crate::BitReader<bool>;
#[doc = "Field `EESAVE` writer - EEPROM Save"]
pub type EESAVE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SYSCFG0_SPEC, bool, O>;
#[doc = "Field `RSTPINCFG` reader - Reset Pin Configuration"]
pub type RSTPINCFG_R = crate::BitReader<RSTPINCFG_A>;
#[doc = "Reset Pin Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTPINCFG_A {
    #[doc = "0: GPIO mode"]
    GPIO = 0,
    #[doc = "1: Reset mode"]
    RST = 1,
}
impl From<RSTPINCFG_A> for bool {
    #[inline(always)]
    fn from(variant: RSTPINCFG_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTPINCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTPINCFG_A {
        match self.bits {
            false => RSTPINCFG_A::GPIO,
            true => RSTPINCFG_A::RST,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == RSTPINCFG_A::GPIO
    }
    #[doc = "Checks if the value of the field is `RST`"]
    #[inline(always)]
    pub fn is_rst(&self) -> bool {
        *self == RSTPINCFG_A::RST
    }
}
#[doc = "Field `RSTPINCFG` writer - Reset Pin Configuration"]
pub type RSTPINCFG_W<'a, const O: u8> = crate::BitWriter<'a, u8, SYSCFG0_SPEC, RSTPINCFG_A, O>;
impl<'a, const O: u8> RSTPINCFG_W<'a, O> {
    #[doc = "GPIO mode"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(RSTPINCFG_A::GPIO)
    }
    #[doc = "Reset mode"]
    #[inline(always)]
    pub fn rst(self) -> &'a mut W {
        self.variant(RSTPINCFG_A::RST)
    }
}
#[doc = "Field `CRCSRC` reader - CRC Source"]
pub type CRCSRC_R = crate::FieldReader<u8, CRCSRC_A>;
#[doc = "CRC Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRCSRC_A {
    #[doc = "0: The CRC is performed on the entire Flash (boot, application code and application data section)."]
    FLASH = 0,
    #[doc = "1: The CRC is performed on the boot section of Flash"]
    BOOT = 1,
    #[doc = "2: The CRC is performed on the boot and application code section of Flash"]
    BOOTAPP = 2,
    #[doc = "3: Disable CRC."]
    NOCRC = 3,
}
impl From<CRCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CRCSRC_A) -> Self {
        variant as _
    }
}
impl CRCSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCSRC_A {
        match self.bits {
            0 => CRCSRC_A::FLASH,
            1 => CRCSRC_A::BOOT,
            2 => CRCSRC_A::BOOTAPP,
            3 => CRCSRC_A::NOCRC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == CRCSRC_A::FLASH
    }
    #[doc = "Checks if the value of the field is `BOOT`"]
    #[inline(always)]
    pub fn is_boot(&self) -> bool {
        *self == CRCSRC_A::BOOT
    }
    #[doc = "Checks if the value of the field is `BOOTAPP`"]
    #[inline(always)]
    pub fn is_bootapp(&self) -> bool {
        *self == CRCSRC_A::BOOTAPP
    }
    #[doc = "Checks if the value of the field is `NOCRC`"]
    #[inline(always)]
    pub fn is_nocrc(&self) -> bool {
        *self == CRCSRC_A::NOCRC
    }
}
#[doc = "Field `CRCSRC` writer - CRC Source"]
pub type CRCSRC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, SYSCFG0_SPEC, u8, CRCSRC_A, 2, O>;
impl<'a, const O: u8> CRCSRC_W<'a, O> {
    #[doc = "The CRC is performed on the entire Flash (boot, application code and application data section)."]
    #[inline(always)]
    pub fn flash(self) -> &'a mut W {
        self.variant(CRCSRC_A::FLASH)
    }
    #[doc = "The CRC is performed on the boot section of Flash"]
    #[inline(always)]
    pub fn boot(self) -> &'a mut W {
        self.variant(CRCSRC_A::BOOT)
    }
    #[doc = "The CRC is performed on the boot and application code section of Flash"]
    #[inline(always)]
    pub fn bootapp(self) -> &'a mut W {
        self.variant(CRCSRC_A::BOOTAPP)
    }
    #[doc = "Disable CRC."]
    #[inline(always)]
    pub fn nocrc(self) -> &'a mut W {
        self.variant(CRCSRC_A::NOCRC)
    }
}
impl R {
    #[doc = "Bit 0 - EEPROM Save"]
    #[inline(always)]
    pub fn eesave(&self) -> EESAVE_R {
        EESAVE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Reset Pin Configuration"]
    #[inline(always)]
    pub fn rstpincfg(&self) -> RSTPINCFG_R {
        RSTPINCFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 6:7 - CRC Source"]
    #[inline(always)]
    pub fn crcsrc(&self) -> CRCSRC_R {
        CRCSRC_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - EEPROM Save"]
    #[inline(always)]
    #[must_use]
    pub fn eesave(&mut self) -> EESAVE_W<0> {
        EESAVE_W::new(self)
    }
    #[doc = "Bit 3 - Reset Pin Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn rstpincfg(&mut self) -> RSTPINCFG_W<3> {
        RSTPINCFG_W::new(self)
    }
    #[doc = "Bits 6:7 - CRC Source"]
    #[inline(always)]
    #[must_use]
    pub fn crcsrc(&mut self) -> CRCSRC_W<6> {
        CRCSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Configuration 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg0](index.html) module"]
pub struct SYSCFG0_SPEC;
impl crate::RegisterSpec for SYSCFG0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [syscfg0::R](R) reader structure"]
impl crate::Readable for SYSCFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg0::W](W) writer structure"]
impl crate::Writable for SYSCFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG0 to value 0"]
impl crate::Resettable for SYSCFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
