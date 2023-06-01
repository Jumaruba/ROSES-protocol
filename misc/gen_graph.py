import matplotlib.pyplot as plt 
import pandas as pd 

dataframe = pd.read_csv('../metrics.csv')
fig, ax = plt.subplots() 
timeunits = [i for i in range(0, dataframe["Handoff"].size)]
ax.plot(timeunits, dataframe["Handoff"], label='ROSES')
ax.plot(timeunits, dataframe["Aworset"], label='AWORSET')
ax.axvline(x=20, color='b', linestyle='--')
ax.legend()
ax.set(xlabel='Time units', ylabel='Memory in bytes', title='Memory usage over time')
ax.grid()
ax.set_yticks([i for i in range(0, 7000, 500)])
plt.savefig('../memory.pdf')
