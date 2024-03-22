circom_p = 21888242871839275222246405745257275088548364400416034343698204186575808495617
our_p = 89
ZKField = FiniteField(our_p)
#59 | 71^10-1

n = 21

q_l_values = [ 0,  1,  1, -1, -1, -1,  0,  1,  1, -1, -1, -1,  0 ]
q_r_values = [ 0,  2,  4,  0,  0,  0, -1,  2,  4,  0,  0,  0, -1 ]
q_o_values = [ 0, -1, -1,  0,  0,  0,  0, -1, -1,  0,  0,  0,  0 ]
q_m_values = [ 1,  0,  0,  1,  1,  1,  1,  0,  0,  1,  1,  1,  1 ]
q_c_values = [-n,  0,  0,  0,  0,  0, -1,  0,  0,  0,  0,  0, -1 ]

r = 7
s = 3


def compute_input_output_poly_values(secret_factor):
   r0 = secret_factor % 2
   r1 = (secret_factor // 2) % 2
   r2 = (secret_factor // 4) % 2
   
   r01 = r0 + r1*2

   secret_factor_minus_1_inverse = ZKField(secret_factor - 1)^(-1)
   
   return ([r0, r01, r0, r1, r2, secret_factor], [r1, r2, r0, r1, r2, secret_factor_minus_1_inverse] , [r01, secret_factor, 0,0,0,0] )

r_input_output_values = compute_input_output_poly_values(r)
s_input_output_values = compute_input_output_poly_values(s)
left_input_values = [r] + r_input_output_values[0] + s_input_output_values[0]
right_input_values = [s] + r_input_output_values[1] + s_input_output_values[1]

c_output_values = [0] + r_input_output_values[2] + s_input_output_values[2]

x_values = range(1, 14)

PolysOnZKField.<x> = PolynomialRing(ZKField)

Qlx = PolysOnZKField.lagrange_polynomial(zip(x_values,q_l_values))
Qrx = PolysOnZKField.lagrange_polynomial(zip(x_values,q_r_values))
Qox = PolysOnZKField.lagrange_polynomial(zip(x_values,q_o_values))
Qmx = PolysOnZKField.lagrange_polynomial(zip(x_values,q_m_values))
Qcx = PolysOnZKField.lagrange_polynomial(zip(x_values,q_c_values))

ax  = PolysOnZKField.lagrange_polynomial(zip(x_values,left_input_values))
bx  = PolysOnZKField.lagrange_polynomial(zip(x_values,right_input_values))
cx  = PolysOnZKField.lagrange_polynomial(zip(x_values,c_output_values))

#Q_l(x)*a(x) + Q_r(x)*b(x) + Q_o(x)* c(x) + Q_m(x)*a(x)*b(x) + Q_c(x) = 0
trace_poly =  Qlx * ax + Qrx*bx+ Qox * cx + Qmx*ax*bx + Qcx  

zero_test_poly = prod(list(map(lambda x_val: x - x_val,x_values)))

print("Trace polynomial: ", trace_poly)
print("Zero test polynomial:", zero_test_poly)
print("The remainder of trace/zerotest: ",trace_poly % zero_test_poly)

qx = trace_poly / zero_test_poly
print("q(x):",qx)

# y^2 = x^3 + 8*x + 10 embeding degree 8 on 101 order 89
# (trace_poly(r) - trace_poly(tau)) / (r - tau)
