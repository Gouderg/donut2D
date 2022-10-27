# Donut 3D render in 2D.

![](donut.gif)

## Draw the donut in the 3D space.

$$
  (ox, oy, z) = (R_2, 0, 0) + (R_1\cos\theta, R_1\sin\theta, 0)
$$

## (x, y, z) in the 3D space after all rotation.

$$
  \begin{pmatrix}x \\ y \\ z\end{pmatrix} = 
  \begin{pmatrix}
    ox * (\cos B \cos\phi + \sin A \sin B \sin\phi) - oy * \cos A * \sin B \\
    ox * (\cos\phi \sin B - \cos B \sin A \sin\phi) + oy * \cos A * \cos B \\
    ox * \cos A \sin\phi + oy * \sin A
  \end{pmatrix}$$

## Calculate the luminance.

$$
  L = \cos\phi \cos\theta \sin B - \cos A \cos\theta \sin\phi - \sin A \sin\theta + \cos B(\cos A \sin\theta - \cos\theta \sin A \sin\phi)
$$


Thanks to [a1k0n](https://www.a1k0n.net/2011/07/20/donut-math.html) for his explanation.