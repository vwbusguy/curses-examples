#!/usr/bin/env python
# -*- coding: utf-8 -*-
import curses,sys,string,random,datetime,signal

basetime = datetime.datetime.now()
att = 0

def update_time(screen):
	global basetime
	now = datetime.datetime.now()
	time = str(now - basetime)
	screen.addstr(3,2,"It was been %s already... " % time)

def getRand():
	letter = ''
	while letter == 'q' or letter == '':
		letter = random.choice(string.letters).lower()
	return letter
	

def listen(screen):
	global att
	t = getRand()
	c = screen.getch()
	if c == ord('q'):
		return False
	elif c == -1:
		return True
	elif c == ord(t):
		screen.addstr(12,25,"You've found the secret foo!")
		screen.addstr(13,25,'It is %s!!!' % t)
		screen.nodelay(0)
		while screen.getch().lower() != ord('q'):
			screen.addstr(14,25,"Congrats!  Press 'q' to exit!")
		return False
	else:
		screen.addstr(13,25,'It was %s!!!' % t)
                att = att + 1
                tries(screen,att)
		curses.beep()
		return True

def tries(screen,amount):
	screen.addstr(5,2,"You've tried %d times to find the missing foo!" % amount)

try:
	s = signal.signal(signal.SIGINT, signal.SIG_IGN)
	screen = curses.initscr()
	curses.noecho()
	screen.nodelay(1)
	curses.curs_set(0)
	screen.border()
	screen.addstr(1,1,'Find the secret foo')
	screen.addstr(12,25,'Can you find me?!?!')
	size = screen.getmaxyx()
	screen.addstr(size[0]-2,1,"Press 'q' to exit")
	while listen(screen):
		update_time(screen)
		screen.refresh()
except Exception as e:
	sys.exit('Something broke!\n' + e.message)
finally:
	signal.signal(signal.SIGINT, s)
	curses.endwin()
