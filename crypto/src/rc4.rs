use core::marker::PhantomData;
use stream_cipher::generic_array::typenum::U256;
use stream_cipher::generic_array::ArrayLength;
use stream_cipher::generic_array::GenericArray;
use stream_cipher::NewStreamCipher;
use stream_cipher::StreamCipher;

/* RC4 */

pub struct RC4<M: ArrayLength<u8>, N: ArrayLength<u8>> {
    _k: PhantomData<GenericArray<u8, M>>,
    _v: PhantomData<GenericArray<u8, N>>,
    sbox: GenericArray<u8, U256>,
}

impl<M, N> NewStreamCipher for RC4<M, N>
where
    M: ArrayLength<u8>,
    N: ArrayLength<u8>,
{
    type KeySize = M;
    type NonceSize = N;

    fn new(key: &GenericArray<u8, Self::KeySize>, iv: &GenericArray<u8, Self::NonceSize>) -> Self {
        let mut sbox: Vec<u8> = (0..=255).map(|i| i).collect();
        let mut kbox: Vec<u8> = vec![];
        let mut j: u8 = 0;
        let mut t: u8;

        while kbox.len() < 256 {
            kbox.extend_from_slice(iv.as_slice());
            kbox.extend_from_slice(key.as_slice());
        }

        for i in 0..=255 {
            j = j.wrapping_add(sbox[i]);
            j = j.wrapping_add(kbox[i]);

            t = sbox[i];
            sbox[i] = sbox[j as usize];
            sbox[j as usize] = t;
        }

        RC4 {
            sbox: *GenericArray::from_slice(&sbox[0..256]),
            _k: PhantomData,
            _v: PhantomData,
        }
    }
}

impl<M, N> StreamCipher for RC4<M, N>
where
    M: ArrayLength<u8>,
    N: ArrayLength<u8>,
{
    fn encrypt(&mut self, data: &mut [u8]) {
        let mut sbox: Vec<u8> = self.sbox.to_vec();
        let mut i: u8 = 0;
        let mut j: u8 = 0;
        let mut t: u8;

        for e in data.iter_mut() {
            i = i.wrapping_add(1);
            j = j.wrapping_add(sbox[i as usize]);

            t = sbox[i as usize];
            sbox[i as usize] = sbox[j as usize];
            sbox[j as usize] = t;

            t = sbox[sbox[i as usize].wrapping_add(sbox[j as usize]) as usize];
            *e ^= t;
        }
    }

    fn decrypt(&mut self, data: &mut [u8]) {
        self.encrypt(data)
    }
}
