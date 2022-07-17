//! In Module 1, we discussed Block ciphers like AES. Block ciphers have a fixed length input.
//! Real wold data that we wish to encrypt _may_ be exactly the right length, but is probably not.
//! When your data is too short, you can simply pad it up to the correct length.
//! When your data is too long, you have some options.
//! 
//! In this exercise, we will explore a few of the common ways that large pieces of data can be broken
//! up and combined in order to encrypt it with a fixed-length block cipher.
//! 
//! WARNING: ECB MODE IS NOT SECURE.
//! Seriously, ECB is NOT secure. Don't use it irl. We are implementing it here to understand _why_ it
//! is not secure and make the point that the most straight-forward approach isn't always the best, and
//! can sometimes be trivially broken.

use aes::Aes128;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

///We're using AES 128 which has 16-byte (128 bit) blocks.
const BLOCK_SIZE: usize = 16;

fn main(){todo!("Maybe this should be a libraray crate. TBD")}

/// Simple AES encryption
/// Helper function to make the core AES block cipher easier to understand.
fn aes_encrypt(data: [u8; 16], key: &[u8; 16]) -> [u8; 16] {

	// Convert the inputs to the necessary data type
	let mut block = GenericArray::from(data);
	let key = GenericArray::from(*key);

	let cipher = Aes128::new(&key);

	cipher.encrypt_block(&mut block);

	block.into()
} 

/// Simple AES encryption
/// Helper function to make the core AES block cipher easier to understand.
fn aes_decrypt(data: [u8; 16], key: &[u8; 16]) -> [u8; 16] {

	// Convert the inputs to the necessary data type
	let mut block = GenericArray::from(data);
	let key = GenericArray::from(*key);

	let cipher = Aes128::new(&key);

	cipher.decrypt_block(&mut block);

	block.into()
}

/// Before we can begin encrypting our raw data, we need it to be a multiple of the
/// block length which is 16 bytes (128 bits) in AES128.
/// 
/// The padding algorithm here is actually not trivial. The trouble is that if we just
/// naively throw a bunch of zeros on the end, ther is no way to know, later, whether
/// those zeros are padding, or part of the message, or some of each.
/// 
/// The scheme works like this. If the data is not a multiple of the block length,  we
/// compute how many pad bytes we need, and then write that number into the last severl bytes.
/// Later we look at the last byte, and remove that number of bytes.
/// 
/// But if the data _is_ a multiple of the block length, then we have a problem. We don't want
/// to later look at the last byte and remove part of the data. Instead, in this case, we add
/// another entire block containing the block length in each byte. In our case,
/// [16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16]
/// 
/// This function also chops the data into blocks for us.
fn pad(mut data: Vec<u8>) -> Vec<[u8; 16]> {
	// When twe have a multiple the second term is 0
	let number_pad_bytes = BLOCK_SIZE - data.len() % BLOCK_SIZE;

	for _ in 0..number_pad_bytes {
		data.push(number_pad_bytes as u8)
	}

	let mut blocks = Vec::new();
	let mut i = 0;
	while i < data.len() {
		let mut block: [u8; BLOCK_SIZE] = Default::default();
		block.copy_from_slice(&data[i..i + BLOCK_SIZE]);
		blocks.push(block);

		i += BLOCK_SIZE;
	}
	
	blocks
}

/// Does the opposite of the pad function.
fn un_pad(data: Vec<[u8; 16]>) -> Vec<u8>{
	todo!()
}

/// The first mode we will implement is the Electronic Code Book, or ECB mode. 
/// Warning: THIS MODE IS NOT SECURE!!!!
///
/// This is probably the first thing you think of when considering how to encrypt
/// large data. In this mode we simply encrypt each block of data under the same key.
/// One good thing about this mode is that it is parallelizable. But to see why it is
/// insecure look at: https://www.ubiqsecurity.com/wp-content/uploads/2022/02/ECB2.png
fn ecb_encrypt(plain_text: Vec<u8>, key: [u8; 16]) -> Vec<u8> {

	todo!()
}

/// Opposite of ecb_encrypt.
fn ecb_decrypt(cipher_text: Vec<u8>, key: [u8; 16]) -> Vec<u8> {
	todo!()
}


/// The next mode, which you can implement on your own is cipherblock chaining.
/// This mode actually is secure, and it often used in real world applications.
/// 
/// In this mode, the ciphertext from the first block is XORed with the
/// plaintext of the next block before it is encrypted.
fn cbc_encrypt(plain_text: Vec<u8>, key: [u8; 16]) -> Vec<u8> {

	// Remember to generate a random initialization vector for the first block.

	todo!()

}

fn cbc_decrypt(cipher_text: Vec<u8>, key: [u8; 16]) -> Vec<u8> {
	todo!()
}