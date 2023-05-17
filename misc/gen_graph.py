import matplotlib.pyplot as plt 
import pandas as pd 

dataframe = pd.read_csv('../metrics.csv')
fig, ax = plt.subplots() 
timeunits = [i for i in range(0, dataframe["Handoff"].size)]
ax.plot(timeunits, dataframe["Handoff"], label='Metrics')
ax.plot(timeunits, dataframe["Aworset"], label='Metrics')
plt.show()
# Add wins set com media pra 100 
# ir até 40 unidade de tempo e depois de 30 parar de fazer operações. 
# 40 clientes, 2 e 8 servers. Fazer gráficos para as duas situações de servers. 
# buscar o número de bytes da estrutura. 