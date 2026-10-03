/*
Manually written memory map of the STM32H753ZI from the reference manual RM0433.
Generic template:
MEMORY
{
	NAME : ORIGIN = 0xADDRESS, LENGTH = SIZE
}
SIZE can be written as 128K, 2M or in hex such as 0x20000
Minimally viable memory map has FLASH and RAM
Linker needs those two regions, all other regions are inactive
until something is put inside them.
All addresses are taken from RM0433.
fish is used for calculating the size:
math "(end - start + 1)/1024" to get it in KiB
*/
/* 
H753 has dual FLASH banks. RM0433, page 153
I will only map the first one, to keep the second free for updates.
SRAM is described as AXI SRAM, page 131. General RAM, DMA can reach it.
*/
MEMORY
{
	FLASH : ORIGIN = 0x08000000, LENGTH = 1M
	RAM : ORIGIN = 0x24000000, LENGTH = 512K
}
