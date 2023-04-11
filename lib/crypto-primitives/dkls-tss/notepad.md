# A Formula

Sure, here is an example of Correlated Oblivious Transfer Extension (COT Extension) with concrete numbers in GF(17):

Suppose Alice has a set of messages {m1, m2, m3, m4} = {5, 9, 12, 14} and wants to send a subset of them to Bob, where the selected messages are the ones that are even. However, Alice does not want to reveal anything about the unselected messages to Bob. To achieve this, Alice and Bob can use the COT Extension protocol with a correlation function that checks if the selected messages are even.

The COT Extension protocol proceeds as follows:

Alice generates a one-time pad key (k1, k2, k3, k4) = (2, 6, 3, 1) in GF(17) and encrypts each message using the pad key:

$m'_1 = m_1 \oplus k_1 = 5 \oplus 2 = 7$

$m'_2 = m_2 \oplus k_2 = 9 \oplus 6 = 15$

$m'_3 = m_3 \oplus k_3 = 12 \oplus 3 = 15$

$m'_4 = m_4 \oplus k_4 = 14 \oplus 1 = 15$

---

Alice sends the encrypted messages to Bob:

$\{m'_1, m'_2, m'_3, m'_4\} = \{7, 15, 15, 15\}$ 

---

Bob selects a random bit for each message, and sends the bit values back to Alice:

$b1 = 1, b2 = 0, b3 = 1, b4 = 0$

---

Alice XORs the encrypted messages with the random bits Bob sent her and sends the result to Bob:

$c_1 = m'_1 \oplus b_1 = 7 \oplus 1 = 6$

$c_2 = m'_2 \oplus b_2 = 15 \oplus 0 = 15$

$c_3 = m'_3 \oplus b_3 = 15 \oplus 1 = 14$

$c_4 = m'_4 \oplus b_4 = 15 \oplus 0 = 15$

---

Bob applies the correlation function to the XORed values and obtains the selected messages that are even:

$Selected = {c_i \mid b_i = 0} = {15}$

---

In this case, no message is selected since none of them are even. However, if Alice had used a correlation function that selected messages with an odd value, Bob would have received {6, 14}, which are the messages that are not even.

Bob can then use the one-time pad key to decrypt the selected messages:

$m_i = c_i \oplus k_i$

---

In this example, there are no selected messages, so Bob cannot decrypt any messages. But if there were selected messages, Bob would be able to use the one-time pad key to decrypt them.

In summary, COT Extension allows Alice to send a subset of messages to Bob that are correlated according to the correlation function (i.e., even messages), while still preserving the privacy of the unselected messages.