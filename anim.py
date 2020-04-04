from Tkinter import * 
import math 
from matrices import Matrix
from poly import Cube

root = Tk()
canvas_width = 800
canvas_height = 600
canvas = Canvas(root, width=canvas_width, height=canvas_height)
canvas.pack()

orientation = Matrix([[1,0,0],[0,1,0],[0,0,1]])
conversion = Matrix([[math.sqrt(3)/2,0.5],[0,-1],[-math.sqrt(3)/2,0.5]])
origin = [canvas_width/2, canvas_height/2]

# finds the positions of everything in 3d space
def moveStuff(obj):
	obj.translate(1,1,0)
	obj.scale(1.001)
	obj.rotate(math.pi/24, 0, math.pi/48)

# converts positions to 2d and draws them (should decouple)
def drawStuff(obj):
	canvas.delete("all")
	drawBG()
	drawAxes()
	drawPoly(obj)

def drawBG():
	canvas.create_rectangle(0, 0, canvas_width, canvas_height, fill="black")

def drawAxes():
	for i in range(0, 3):
		point = Matrix.mtimes(conversion,orientation).data[i]
		point[0] *= 200
		point[1] *= 200
		drawLine([0,0], point)

def drawPoint(point):
	canvas.create_rectangle(point[0]+origin[0]-2, point[1]+origin[1]-2, 
		point[0]+origin[0]+2, point[1]+origin[1]+2, fill="white")

def drawLine(start, end):
	canvas.create_line(origin[0]+start[0], origin[1]+start[1],
		origin[0]+end[0], origin[1]+end[1], fill="white")

def drawPoly(obj):
	coords = Matrix.mtimes(conversion,obj.verts)
	for v in coords.data:
		drawPoint(v)
	for e in obj.edges:
		print obj.verts.data[e[0]]
		start = coords.data[e[0]]
		end = coords.data[e[1]]
		drawLine(start, end)

def doStuff(obj):
	moveStuff(obj)
	drawStuff(obj)
	canvas.after(1000/24, lambda: doStuff(obj))

def main():
	# want a list of all objects in the scene to do stuff with 
	cube = Cube()
	cube.scale(100)
	doStuff(cube)
	mainloop()

if __name__=="__main__":
	main()