import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D

# Симуляция поля Q(x) (примитивный генератор)
def generate_Q_field(N, amp=1):
    # Q(x) = q0 + i*q1 + j*q2 + k*q3, на S^3
    phi1 = np.random.uniform(0, 2*np.pi, N)
    phi2 = np.random.uniform(0, 2*np.pi, N)
    q0 = amp * np.cos(phi1)
    q1 = amp * np.sin(phi1) * np.cos(phi2)
    q2 = amp * np.sin(phi1) * np.sin(phi2)
    q3 = amp * np.sin(phi2)
    return np.stack([q0, q1, q2, q3], axis=1)

N = 1000
Q = generate_Q_field(N)

# 3D scatter plot для отображения конфигураций Q(x) на S^3 (проекция в 3D)
fig = plt.figure()
ax = fig.add_subplot(111, projection='3d')
ax.scatter(Q[:,1], Q[:,2], Q[:,3], c=Q[:,0], cmap='viridis', alpha=0.8)
ax.set_xlabel('q1')
ax.set_ylabel('q2')
ax.set_zlabel('q3')
plt.title('Кватернионное поле Q(x) (S^3), цвет: q0')
plt.show()

# Пример: визуализация топологического кольца (Хопф)
t = np.linspace(0, 2*np.pi, N)
x = np.cos(t)
y = np.sin(t)
z = np.sin(2*t)
fig2 = plt.figure()
ax2 = fig2.add_subplot(111, projection='3d')
ax2.plot(x, y, z, color='red', linewidth=2)
plt.title('Топологический хопф-объект (Hopfion)')
plt.show()
