use stream_cipher::generic_array::typenum::UTerm;
use stream_cipher::generic_array::ArrayLength;
use stream_cipher::generic_array::GenericArray;
use stream_cipher::NewStreamCipher;
use stream_cipher::StreamCipher;

// RC4

pub struct RC4<N: ArrayLength<u8>> {
    key: GenericArray<u8, N>,
}

impl<N> NewStreamCipher for RC4<N>
where
    N: ArrayLength<u8>,
{
    type KeySize = N;
    type NonceSize = UTerm;

    fn new(key: &GenericArray<u8, Self::KeySize>, _: &GenericArray<u8, Self::NonceSize>) -> Self {
        RC4 { key: key.clone() }
    }
}

impl<N> StreamCipher for RC4<N>
where
    N: ArrayLength<u8>,
{
    fn encrypt(&mut self, data: &mut [u8]) {
        let mut sbox: Vec<u8> = (0..=255).map(|i| i).collect();
        let mut kbox: Vec<u8> = vec![];
        let mut i: u8;
        let mut j: u8;
        let mut t: u8;

        kbox.extend_from_slice(self.key.as_slice());
        j = 0;

        for i in 0..=255 {
            // j = (j + S_i + K_i) mod 256
            j = j.wrapping_add(sbox[i]);
            j = j.wrapping_add(kbox[i % kbox.len()]);

            // swap S_i and S_j
            t = sbox[i];
            sbox[i] = sbox[j as usize];
            sbox[j as usize] = t;
        }

        i = 0;
        j = 0;

        for e in data.iter_mut() {
            // i = (i + 1) mod 256
            i = i.wrapping_add(1);
            j = j.wrapping_add(sbox[i as usize]);

            // swap S_i and S_j
            t = sbox[i as usize];
            sbox[i as usize] = sbox[j as usize];
            sbox[j as usize] = t;

            // k = (S_i + S_j) mod 256
            // R = S_k
            t = sbox[sbox[i as usize].wrapping_add(sbox[j as usize]) as usize];
            *e ^= t;
        }
    }

    fn decrypt(&mut self, data: &mut [u8]) {
        self.encrypt(data)
    }
}
