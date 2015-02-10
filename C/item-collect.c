/*
 * item-collect.c
 * 
 * Copyright 2015 Scott Williams <vwbusguy@fedoraproject.org>
 * 
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
 * MA 02110-1301, USA.
 * 
 * 
 */


#include <unistd.h>
#include <ncurses.h>

int main(int argc, char **argv)
{
	// Initial params
	int rand();
	void drawPerson(), drawItem();
	int ix = 5, iy = 5;
	int px = 1, py = 2;
	int max_y = 0, max_x = 0;
	int cnt = 0;
	char scnt[20] = "0";
	
	// Initialize the curses screen
	initscr();
	keypad(stdscr, TRUE);
	noecho();
	curs_set(FALSE);
	
	// Draw initial screen
	mvprintw(0, 0, "Get the item!");
	drawPerson(px,py);
	drawItem(ix,iy);
	refresh();
	
	// Main loop
	while (TRUE){
		
		// Get user input and act on it
		int ch = wgetch(stdscr);
		getmaxyx(stdscr,max_y,max_x);
		
		// This could be a switch, but left for illustration
		if ((ch == KEY_LEFT) && (px > 0 )){
			px = px -1 ;
		}else if ((ch == KEY_RIGHT) && (px < (max_x - 1))){
			px++;
		}else if ((ch == KEY_UP) && (py > 1 )){
			py = py -1;
		}else if ((ch == KEY_DOWN) && (py < (max_y - 1))){
			py++;
		}else if (ch == 'q'){
			break;
		}
		clear();
		
		// Redraw parameters
		if (((ix + 1) == px) && (iy == py)){
			cnt++;
			snprintf(scnt,20,"%d",cnt);
			ix = rand() % max_x - 3;
			iy = rand() % max_y;
			if (iy < 2 ) iy = 2;
			mvprintw(0,0, "Nice!");
		}else{
			mvprintw(0,0, "Get the item!");
		}

		// Update the screen
		mvprintw(max_y-1,0,"Press 'q' to exit");
		mvprintw(0,max_x-3,scnt);
		drawItem(ix,iy);
		drawPerson(px,py);
		refresh();
	}
	
	// Clean up and exit
	endwin();
	return 0;
}

// Draw the Item
void drawItem(int x, int y){
	mvprintw(y,x,"[*]");
	mvprintw(y-1,x,"=^=");
}

// Draw the person
void drawPerson(int x, int y){
	mvprintw(y-1,x,"o");
	mvprintw(y,x,"X");
}
