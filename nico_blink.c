// nico_blink.c

// For rcc_periph_clock_enable
#define _RCC_REG(i)             MMIO32(RCC_BASE + ((i) >> 5))
#define _RCC_BIT(i)             (1 << ((i) & 0x1f))
//For gpio_mode_setup
#define GPIO_MODER(port)                MMIO32((port) + 0x00)
#define GPIO_PUPDR(port)                MMIO32((port) + 0x0c)
#define GPIO_MODE(n, mode)              ((mode) << (2 * (n)))
#define GPIO_PUPD(n, pupd)              ((pupd) << (2 * (n)))
//For gpio_set
#define GPIO_BSRR(port)                 MMIO32((port) + 0x18)
//For gpio_toggle
#define GPIO_ODR(port)                  MMIO32((port) + 0x14)

void rcc_periph_clock_enable(enum rcc_periph_clken clken)
{
        _RCC_REG(clken) |= _RCC_BIT(clken);
}

void gpio_mode_setup(uint32_t gpioport, uint8_t mode, uint8_t pull_up_down,
                     uint16_t gpios)
{
        uint16_t i;
        uint32_t moder, pupd;
 
        /*
         * We want to set the config only for the pins mentioned in gpios,
         * but keeping the others, so read out the actual config first.
         */
        moder = GPIO_MODER(gpioport);
        pupd = GPIO_PUPDR(gpioport);
 
        for (i = 0; i < 16; i++) {
                if (!((1 << i) & gpios)) {
                        continue;
                }
 
                moder &= ~GPIO_MODE_MASK(i);
                moder |= GPIO_MODE(i, mode);
                pupd &= ~GPIO_PUPD_MASK(i);
                pupd |= GPIO_PUPD(i, pull_up_down);
        }
 
        /* Set mode and pull up/down control registers. */
        GPIO_MODER(gpioport) = moder;
        GPIO_PUPDR(gpioport) = pupd;
}

void gpio_set(uint32_t gpioport, uint16_t gpios)
{
        GPIO_BSRR(gpioport) = gpios;
}

void  gpio_clear(uint32_t gpioport, uint16_t gpios)
{
        GPIO_BSRR(gpioport) = (gpios << 16);
}

void gpio_toggle(uint32_t gpioport, uint16_t gpios)
{
        uint32_t port = GPIO_ODR(gpioport);
        GPIO_BSRR(gpioport) = ((port & gpios) << 16) | (~port & gpios);
}


// Makes the Tartan Artibeus EXPT board LEDs blink
//
// Written by Bradley Denby
// Other contributors: None
//
// See the top-level LICENSE file for the license.

// libopencm3

int main(void) {
  rcc_periph_clock_enable(RCC_GPIOC);
  gpio_mode_setup(GPIOC, GPIO_MODE_OUTPUT, GPIO_PUPD_NONE, GPIO10);
  gpio_mode_setup(GPIOC, GPIO_MODE_OUTPUT, GPIO_PUPD_NONE, GPIO12);
  gpio_set(GPIOC, GPIO10);
  gpio_clear(GPIOC, GPIO12);
  while(1) {
    for(int i=0; i<400000; i++) {
      __asm__("nop"); //asm is the only one I couldn't find
    }
    gpio_toggle(GPIOC, GPIO10);
    gpio_toggle(GPIOC, GPIO12);
  }
}
