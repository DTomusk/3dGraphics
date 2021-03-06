from tkinter import * 
import math 
from matrices import Matrix
from poly import *

root = Tk()
canvas_width = 1200
canvas_height = 800
canvas = Canvas(root, width=canvas_width, height=canvas_height)
canvas.pack()

orientation = Matrix([[1,0,0],[0,1,0],[0,0,1]])
forientation = Matrix([[1,0,0,0],[0,1,0,0],[0,0,1,0],[0,0,0,1]])

conversion = Matrix([[math.sqrt(3)/2,0.5],[0,-1],[-math.sqrt(3)/2,0.5]])
forversion = Matrix([[0,1],[1,0],[0,-1],[-1,0]])
origin = [canvas_width/2, canvas_height/2]

# finds the positions of everything in 3d space
def moveStuff(obj):
	#global origin
	#origin[0]-=1
	#origin[1]-=1
	global forientation
	#forientation = Matrix.fortate(forientation, math.pi/32, 0, 0, 0, 0, 0)
	#obj.tr4nslate(1,0,0,0)
	#obj.scale(1.01)
	obj.fortate(0, math.pi/24, 0, math.pi/24, 0, math.pi/24)

# takes a poly and produces a new one whose attributes match the 2d coordinates 
def convert(obj):
	verts = Matrix.mtimes(forversion,Matrix.mtimes(forientation, obj.verts))
	for v in verts.data:
		v[0] += origin[0]
		v[1] += origin[1]
	edges = obj.edges
	faces = obj.faces
	center = Matrix.mtimes(forversion,Matrix.mtimes(forientation, obj.center))
	for c in center.data:
		c[0] += origin[0]
		c[1] += origin[1]
	return Poly(verts, edges, faces, center)

def drawStuff(obj):
	# clear canvas every frame
	canvas.delete("all")
	drawBG()
	#draw4xes()
	# draw everything in the scene 
	drawPoly(obj)

def drawBG():
	canvas.create_rectangle(0, 0, canvas_width, canvas_height, fill="black")

def drawAxes():
	# here I'm converting manually because I don't expect to be converting many non polys
	for i in range(0, 3):
		point = Matrix.mtimes(conversion,orientation).data[i]
		point[0] *= 200
		point[0] += origin[0]
		point[1] *= 200
		point[1] += origin[1]
		drawLine(origin, point)

def draw4xes():
	for i in range(0, 4):
		point = Matrix.mtimes(forversion,forientation).data[i]
		point[0] *= 200
		point[0] += origin[0]
		point[1] *= 200
		point[1] += origin[1]
		drawLine(origin, point)

# draws a 2x2 rectangle
def drawPoint(point):
	canvas.create_rectangle(point[0]-2, point[1]-2, 
		point[0]+2, point[1]+2, fill="white")

def drawLine(start, end):
	canvas.create_line(start[0], start[1],
		end[0], end[1], fill="white")

def drawPoly(obj):
	toDraw = convert(obj)
	# this is dumb, mtimes on a vector should return a vector 
	# I keep having to say .data[], maybe that should be handled in drawPoint? 
	drawPoint(toDraw.center.data[0])
	for v in toDraw.verts.data:
		drawPoint(v)
	for e in toDraw.edges:
		start = toDraw.verts.data[e[0]]
		end = toDraw.verts.data[e[1]]
		drawLine(start, end)

# work out what's going on in the scene then render the scene 
def doStuff(obj):
	moveStuff(obj)
	drawStuff(obj)
	# was 1000/12 before, but need int
	canvas.after(83, lambda: doStuff(obj))

def main():
	# want a list of all objects in the scene to do stuff with 
	#cube = Cube()
	#tesseract = Hypercube()
	#cube.scale(100)
	#tesseract.sc4le(150)

	#fivecell = FiveCell()
	#fivecell.sc4le(100)

	sixteen = Hexadecachoron()
	sixteen.sc4le(150)

	# eventually doStuff will pass a whole scene and draw all of the objects there
	# but for now a cube is good
	doStuff(sixteen)
	mainloop()

if __name__=="__main__":
	main()