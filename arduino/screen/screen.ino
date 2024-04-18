#include "Adafruit_GFX.h"
#include "MCUFRIEND_kbv.h"

#define BLACK 0x0000
#define NAVY 0x000F
#define DARKGREEN 0x03E0
#define DARKCYAN 0x03EF
#define MAROON 0x7800
#define PURPLE 0x780F
#define OLIVE 0x7BE0
#define LIGHTGREY 0xC618
#define DARKGREY 0x7BEF
#define BLUE 0x001F
#define GREEN 0x07E0
#define CYAN 0x07FF
#define RED 0xF800
#define MAGENTA 0xF81F
#define YELLOW 0xFFE0
#define WHITE 0xFFFF
#define ORANGE 0xFD20
#define GREENYELLOW 0xAFE5
#define PINK 0xF81F

MCUFRIEND_kbv tft;

uint16_t id;
uint16_t width;
uint16_t height;

void setup() {
	id = tft.readID();

  	tft.reset();
  	tft.begin(id);

	width = tft.width();
	height = tft.height();

	tft.setRotation(1);	// 0=0deg, 1=90deg, 2=180deg, 3=270deg
	tft.fillScreen(NAVY);
}

void loop() {
	int colour = 0xFFFF;

	for (int i = 0; i < width; i = i + 10) {
		tft.drawLine(0, 0, height, i, colour--);
	}

	colour = 0x0000;

	for (int i = width; i >= 0; i = i - 10) {
		tft.drawLine(height, width, 0, i, colour++);
	}

	tft.setCursor(0, 0);
	tft.setTextColor(WHITE);
	tft.setTextSize(2);	// A number between 1 and 5
	tft.println("(defun hello ()");
	tft.println("  (format t \"Hello, world!\"))");
	tft.println("(hello)");

	while (true);
}
