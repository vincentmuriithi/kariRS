#[doc = "Register `VREG_CTRL` reader"]
pub struct R(crate::R<VREG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREG_CTRL` writer"]
pub struct W(crate::W<VREG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREG_CTRL_SPEC>;
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
impl From<crate::W<VREG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DVDD_OK` reader - DVDD Supply Voltage Valid"]
pub type DVDD_OK_R = crate::BitReader<DVDD_OK_A>;
#[doc = "DVDD Supply Voltage Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVDD_OK_A {
    #[doc = "0: Digital voltage regulator disabled or supply voltage not stable"]
    DIGITAL_VOLTAGE_REGULATOR_DISABLED_OR_SUPPLY_VOLTAGE_NOT_STABLE = 0,
    #[doc = "1: Digital supply voltage has settled"]
    DIGITAL_SUPPLY_VOLTAGE_HAS_SETTLED = 1,
}
impl From<DVDD_OK_A> for bool {
    #[inline(always)]
    fn from(variant: DVDD_OK_A) -> Self {
        variant as u8 != 0
    }
}
impl DVDD_OK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVDD_OK_A {
        match self.bits {
            false => DVDD_OK_A::DIGITAL_VOLTAGE_REGULATOR_DISABLED_OR_SUPPLY_VOLTAGE_NOT_STABLE,
            true => DVDD_OK_A::DIGITAL_SUPPLY_VOLTAGE_HAS_SETTLED,
        }
    }
    #[doc = "Checks if the value of the field is `DIGITAL_VOLTAGE_REGULATOR_DISABLED_OR_SUPPLY_VOLTAGE_NOT_STABLE`"]
    #[inline(always)]
    pub fn is_digital_voltage_regulator_disabled_or_supply_voltage_not_stable(&self) -> bool {
        *self == DVDD_OK_A::DIGITAL_VOLTAGE_REGULATOR_DISABLED_OR_SUPPLY_VOLTAGE_NOT_STABLE
    }
    #[doc = "Checks if the value of the field is `DIGITAL_SUPPLY_VOLTAGE_HAS_SETTLED`"]
    #[inline(always)]
    pub fn is_digital_supply_voltage_has_settled(&self) -> bool {
        *self == DVDD_OK_A::DIGITAL_SUPPLY_VOLTAGE_HAS_SETTLED
    }
}
#[doc = "Field `DVDD_OK` writer - DVDD Supply Voltage Valid"]
pub type DVDD_OK_W<'a, const O: u8> = crate::BitWriter<'a, u8, VREG_CTRL_SPEC, DVDD_OK_A, O>;
impl<'a, const O: u8> DVDD_OK_W<'a, O> {
    #[doc = "Digital voltage regulator disabled or supply voltage not stable"]
    #[inline(always)]
    pub fn digital_voltage_regulator_disabled_or_supply_voltage_not_stable(self) -> &'a mut W {
        self.variant(DVDD_OK_A::DIGITAL_VOLTAGE_REGULATOR_DISABLED_OR_SUPPLY_VOLTAGE_NOT_STABLE)
    }
    #[doc = "Digital supply voltage has settled"]
    #[inline(always)]
    pub fn digital_supply_voltage_has_settled(self) -> &'a mut W {
        self.variant(DVDD_OK_A::DIGITAL_SUPPLY_VOLTAGE_HAS_SETTLED)
    }
}
#[doc = "Field `DVREG_EXT` reader - Use External DVDD Regulator"]
pub type DVREG_EXT_R = crate::BitReader<DVREG_EXT_A>;
#[doc = "Use External DVDD Regulator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVREG_EXT_A {
    #[doc = "0: Internal DVDD voltage regulator for the digital section is enabled."]
    DVDD_INT = 0,
    #[doc = "1: Internal DVDD voltage regulator is disabled; use external regulated 1.8V supply voltage for the digital section."]
    DVDD_EXT = 1,
}
impl From<DVREG_EXT_A> for bool {
    #[inline(always)]
    fn from(variant: DVREG_EXT_A) -> Self {
        variant as u8 != 0
    }
}
impl DVREG_EXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVREG_EXT_A {
        match self.bits {
            false => DVREG_EXT_A::DVDD_INT,
            true => DVREG_EXT_A::DVDD_EXT,
        }
    }
    #[doc = "Checks if the value of the field is `DVDD_INT`"]
    #[inline(always)]
    pub fn is_dvdd_int(&self) -> bool {
        *self == DVREG_EXT_A::DVDD_INT
    }
    #[doc = "Checks if the value of the field is `DVDD_EXT`"]
    #[inline(always)]
    pub fn is_dvdd_ext(&self) -> bool {
        *self == DVREG_EXT_A::DVDD_EXT
    }
}
#[doc = "Field `DVREG_EXT` writer - Use External DVDD Regulator"]
pub type DVREG_EXT_W<'a, const O: u8> = crate::BitWriter<'a, u8, VREG_CTRL_SPEC, DVREG_EXT_A, O>;
impl<'a, const O: u8> DVREG_EXT_W<'a, O> {
    #[doc = "Internal DVDD voltage regulator for the digital section is enabled."]
    #[inline(always)]
    pub fn dvdd_int(self) -> &'a mut W {
        self.variant(DVREG_EXT_A::DVDD_INT)
    }
    #[doc = "Internal DVDD voltage regulator is disabled; use external regulated 1.8V supply voltage for the digital section."]
    #[inline(always)]
    pub fn dvdd_ext(self) -> &'a mut W {
        self.variant(DVREG_EXT_A::DVDD_EXT)
    }
}
#[doc = "Field `AVDD_OK` reader - AVDD Supply Voltage Valid"]
pub type AVDD_OK_R = crate::BitReader<AVDD_OK_A>;
#[doc = "AVDD Supply Voltage Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVDD_OK_A {
    #[doc = "0: Analog voltage regulator disabled or supply voltage not stable"]
    ANALOG_VOLTAGE_REGULATOR_DISABLED_OR_SUPPLY_VOLTAGE_NOT_STABLE = 0,
    #[doc = "1: Analog supply voltage has settled"]
    ANALOG_SUPPLY_VOLTAGE_HAS_SETTLED = 1,
}
impl From<AVDD_OK_A> for bool {
    #[inline(always)]
    fn from(variant: AVDD_OK_A) -> Self {
        variant as u8 != 0
    }
}
impl AVDD_OK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVDD_OK_A {
        match self.bits {
            false => AVDD_OK_A::ANALOG_VOLTAGE_REGULATOR_DISABLED_OR_SUPPLY_VOLTAGE_NOT_STABLE,
            true => AVDD_OK_A::ANALOG_SUPPLY_VOLTAGE_HAS_SETTLED,
        }
    }
    #[doc = "Checks if the value of the field is `ANALOG_VOLTAGE_REGULATOR_DISABLED_OR_SUPPLY_VOLTAGE_NOT_STABLE`"]
    #[inline(always)]
    pub fn is_analog_voltage_regulator_disabled_or_supply_voltage_not_stable(&self) -> bool {
        *self == AVDD_OK_A::ANALOG_VOLTAGE_REGULATOR_DISABLED_OR_SUPPLY_VOLTAGE_NOT_STABLE
    }
    #[doc = "Checks if the value of the field is `ANALOG_SUPPLY_VOLTAGE_HAS_SETTLED`"]
    #[inline(always)]
    pub fn is_analog_supply_voltage_has_settled(&self) -> bool {
        *self == AVDD_OK_A::ANALOG_SUPPLY_VOLTAGE_HAS_SETTLED
    }
}
#[doc = "Field `AVDD_OK` writer - AVDD Supply Voltage Valid"]
pub type AVDD_OK_W<'a, const O: u8> = crate::BitWriter<'a, u8, VREG_CTRL_SPEC, AVDD_OK_A, O>;
impl<'a, const O: u8> AVDD_OK_W<'a, O> {
    #[doc = "Analog voltage regulator disabled or supply voltage not stable"]
    #[inline(always)]
    pub fn analog_voltage_regulator_disabled_or_supply_voltage_not_stable(self) -> &'a mut W {
        self.variant(AVDD_OK_A::ANALOG_VOLTAGE_REGULATOR_DISABLED_OR_SUPPLY_VOLTAGE_NOT_STABLE)
    }
    #[doc = "Analog supply voltage has settled"]
    #[inline(always)]
    pub fn analog_supply_voltage_has_settled(self) -> &'a mut W {
        self.variant(AVDD_OK_A::ANALOG_SUPPLY_VOLTAGE_HAS_SETTLED)
    }
}
#[doc = "Field `AVREG_EXT` reader - Use External AVDD Regulator"]
pub type AVREG_EXT_R = crate::BitReader<AVREG_EXT_A>;
#[doc = "Use External AVDD Regulator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVREG_EXT_A {
    #[doc = "0: Internal AVDD voltage regulator for the analog section is enabled."]
    AVDD_INT = 0,
    #[doc = "1: Internal AVDD voltage regulator is disabled; use external regulated 1.8V supply voltage for the analog section."]
    AVDD_EXT = 1,
}
impl From<AVREG_EXT_A> for bool {
    #[inline(always)]
    fn from(variant: AVREG_EXT_A) -> Self {
        variant as u8 != 0
    }
}
impl AVREG_EXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVREG_EXT_A {
        match self.bits {
            false => AVREG_EXT_A::AVDD_INT,
            true => AVREG_EXT_A::AVDD_EXT,
        }
    }
    #[doc = "Checks if the value of the field is `AVDD_INT`"]
    #[inline(always)]
    pub fn is_avdd_int(&self) -> bool {
        *self == AVREG_EXT_A::AVDD_INT
    }
    #[doc = "Checks if the value of the field is `AVDD_EXT`"]
    #[inline(always)]
    pub fn is_avdd_ext(&self) -> bool {
        *self == AVREG_EXT_A::AVDD_EXT
    }
}
#[doc = "Field `AVREG_EXT` writer - Use External AVDD Regulator"]
pub type AVREG_EXT_W<'a, const O: u8> = crate::BitWriter<'a, u8, VREG_CTRL_SPEC, AVREG_EXT_A, O>;
impl<'a, const O: u8> AVREG_EXT_W<'a, O> {
    #[doc = "Internal AVDD voltage regulator for the analog section is enabled."]
    #[inline(always)]
    pub fn avdd_int(self) -> &'a mut W {
        self.variant(AVREG_EXT_A::AVDD_INT)
    }
    #[doc = "Internal AVDD voltage regulator is disabled; use external regulated 1.8V supply voltage for the analog section."]
    #[inline(always)]
    pub fn avdd_ext(self) -> &'a mut W {
        self.variant(AVREG_EXT_A::AVDD_EXT)
    }
}
impl R {
    #[doc = "Bit 2 - DVDD Supply Voltage Valid"]
    #[inline(always)]
    pub fn dvdd_ok(&self) -> DVDD_OK_R {
        DVDD_OK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Use External DVDD Regulator"]
    #[inline(always)]
    pub fn dvreg_ext(&self) -> DVREG_EXT_R {
        DVREG_EXT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - AVDD Supply Voltage Valid"]
    #[inline(always)]
    pub fn avdd_ok(&self) -> AVDD_OK_R {
        AVDD_OK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Use External AVDD Regulator"]
    #[inline(always)]
    pub fn avreg_ext(&self) -> AVREG_EXT_R {
        AVREG_EXT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - DVDD Supply Voltage Valid"]
    #[inline(always)]
    #[must_use]
    pub fn dvdd_ok(&mut self) -> DVDD_OK_W<2> {
        DVDD_OK_W::new(self)
    }
    #[doc = "Bit 3 - Use External DVDD Regulator"]
    #[inline(always)]
    #[must_use]
    pub fn dvreg_ext(&mut self) -> DVREG_EXT_W<3> {
        DVREG_EXT_W::new(self)
    }
    #[doc = "Bit 6 - AVDD Supply Voltage Valid"]
    #[inline(always)]
    #[must_use]
    pub fn avdd_ok(&mut self) -> AVDD_OK_W<6> {
        AVDD_OK_W::new(self)
    }
    #[doc = "Bit 7 - Use External AVDD Regulator"]
    #[inline(always)]
    #[must_use]
    pub fn avreg_ext(&mut self) -> AVREG_EXT_W<7> {
        AVREG_EXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage Regulator Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vreg_ctrl](index.html) module"]
pub struct VREG_CTRL_SPEC;
impl crate::RegisterSpec for VREG_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vreg_ctrl::R](R) reader structure"]
impl crate::Readable for VREG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vreg_ctrl::W](W) writer structure"]
impl crate::Writable for VREG_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREG_CTRL to value 0"]
impl crate::Resettable for VREG_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
