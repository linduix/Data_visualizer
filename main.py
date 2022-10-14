#!.\venv\Scripts\python.exe
import matplotlib.pyplot as mp
import csv, sys, subprocess

try:
	path = sys.argv[1]
except IndexError as e:
	print("error: 1 argument expected, none given")
	sys.exit(1)

xpoints = []
ypoints = []
with open(path, 'r') as f:
	csvreader = csv.reader(f)
	for i in csvreader:
		try:
			xpoints.append(float(i[0]))
			if i[1] == '':
				ypoints.append(0.0)
			else:
				ypoints.append(float(i[1]))
		except ValueError as e:
			with open('.\\errorlog.txt', 'w') as el:
				el.write(f"Error: {e}")


mp.xlabel("Income")
mp.ylabel("Happiness")

mp.scatter(xpoints, ypoints, c="black", alpha=0.7)

subprocess.run(["data_processing.exe", path])
with open('bestfitLine.txt', 'r') as f:
	data = f.readline()
	data = data.split(",")
	m = float(data[0])
	b = float(data[1])

maxX = (max(xpoints) - min(xpoints)) * 0.1 + max(xpoints)
predY = m * maxX + b

xline = [0.0, maxX]
yline = [b,  predY]

mp.plot(xline, yline, linestyle=":")

mp.show()