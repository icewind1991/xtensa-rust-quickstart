/* Linker script for the ESP8266 */

MEMORY
{
    /* Use values from the ESP-IDF 'bootloader' component.
    /* TODO: Use human-readable lengths */
    /* TODO: Use the full memory map - this is just a test */
    /* vectors ( RX )       : ORIGIN = 0x40080000, len = 0x400 */
    iram_seg ( RX )       : ORIGIN = 0x40100000, len = 0xFC00
    dram_seg ( RW )       : ORIGIN = 0x3FFE8000, len = 0x1000

}
