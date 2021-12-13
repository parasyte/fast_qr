#[test]
fn structure_codewords_data() {
    const VERSION: crate::comptime::qrcode::version::Version =
        crate::comptime::qrcode::version::Version::V05;
    const QUALITY: crate::comptime::qrcode::ecl::ECL = crate::comptime::qrcode::ecl::ECL::Q;

    let data_codewords = &[
        67, 85, 70, 134, 87, 38, 85, 194, 119, 50, 6, 18, 6, 103, 38, 246, 246, 66, 7, 118, 134,
        242, 7, 38, 86, 22, 198, 199, 146, 6, 182, 230, 247, 119, 50, 7, 118, 134, 87, 38, 82, 6,
        134, 151, 50, 7, 70, 247, 118, 86, 194, 6, 151, 50, 16, 236, 17, 236, 17, 236, 17, 236,
    ]
    .to_vec();
    let error_codewords = crate::comptime::qrcode::hardcode::get_polynomial(VERSION, QUALITY);

    let structure = crate::comptime::qrcode::polynomials::structure(
        &data_codewords,
        &error_codewords,
        QUALITY,
        VERSION,
    );
    let max = VERSION.max_bytes();
    let message = &structure[..data_codewords.len()];
    let _errors = &structure[data_codewords.len()..max];

    assert_eq!(
        message,
        [
            67, 246, 182, 70, 85, 246, 230, 247, 70, 66, 247, 118, 134, 7, 119, 86, 87, 118, 50,
            194, 38, 134, 7, 6, 85, 242, 118, 151, 194, 7, 134, 50, 119, 38, 87, 16, 50, 86, 38,
            236, 6, 22, 82, 17, 18, 198, 6, 236, 6, 199, 134, 17, 103, 146, 151, 236, 38, 6, 50,
            17, 7, 236
        ]
        .to_vec()
    );
}

#[test]
fn structure_codewords_error() {
    const VERSION: crate::comptime::qrcode::version::Version =
        crate::comptime::qrcode::version::Version::V05;
    const QUALITY: crate::comptime::qrcode::ecl::ECL = crate::comptime::qrcode::ecl::ECL::Q;

    let data_codewords = &[
        67, 85, 70, 134, 87, 38, 85, 194, 119, 50, 6, 18, 6, 103, 38, 246, 246, 66, 7, 118, 134,
        242, 7, 38, 86, 22, 198, 199, 146, 6, 182, 230, 247, 119, 50, 7, 118, 134, 87, 38, 82, 6,
        134, 151, 50, 7, 70, 247, 118, 86, 194, 6, 151, 50, 16, 236, 17, 236, 17, 236, 17, 236,
    ]
    .to_vec();
    let error_codewords = crate::comptime::qrcode::hardcode::get_polynomial(VERSION, QUALITY);

    let structure = crate::comptime::qrcode::polynomials::structure(
        &data_codewords,
        &error_codewords,
        QUALITY,
        VERSION,
    );

    let max = VERSION.max_bytes();
    let _message = &structure[..data_codewords.len()];
    let errors = &structure[data_codewords.len()..max];

    assert_eq!(
        errors,
        [
            213, 87, 148, 235, 199, 204, 116, 159, 11, 96, 177, 5, 45, 60, 212, 173, 115, 202, 76,
            24, 247, 182, 133, 147, 241, 124, 75, 59, 223, 157, 242, 33, 229, 200, 238, 106, 248,
            134, 76, 40, 154, 27, 195, 255, 117, 129, 230, 172, 154, 209, 189, 82, 111, 17, 10, 2,
            86, 163, 108, 131, 161, 163, 240, 32, 111, 120, 192, 178, 39, 133, 141, 236
        ]
        .to_vec()
    );
}

#[test]
fn structure_codewords_binary_repr() {
    const VERSION: crate::comptime::qrcode::version::Version =
        crate::comptime::qrcode::version::Version::V05;
    const QUALITY: crate::comptime::qrcode::ecl::ECL = crate::comptime::qrcode::ecl::ECL::Q;

    let data_codewords = &[
        67, 85, 70, 134, 87, 38, 85, 194, 119, 50, 6, 18, 6, 103, 38, 246, 246, 66, 7, 118, 134,
        242, 7, 38, 86, 22, 198, 199, 146, 6, 182, 230, 247, 119, 50, 7, 118, 134, 87, 38, 82, 6,
        134, 151, 50, 7, 70, 247, 118, 86, 194, 6, 151, 50, 16, 236, 17, 236, 17, 236, 17, 236,
    ]
    .to_vec();
    let error_codewords = crate::comptime::qrcode::hardcode::get_polynomial(VERSION, QUALITY);

    let structure = crate::comptime::qrcode::polynomials::structure(
        &data_codewords,
        &error_codewords,
        QUALITY,
        VERSION,
    );

    assert_eq!(
        crate::comptime::qrcode::helpers::binary_to_binarystring_version(structure, VERSION, QUALITY)
        .as_string(),
        "01000011111101101011011001000110010101011111011011100110111101110100011001000010111101110111011010000110000001110111011101010110010101110111011000110010110000100010011010000110000001110000011001010101111100100111011010010111110000100000011110000110001100100111011100100110010101110001000000110010010101100010011011101100000001100001011001010010000100010001001011000110000001101110110000000110110001111000011000010001011001111001001010010111111011000010011000000110001100100001000100000111111011001101010101010111100101001110101111000111110011000111010010011111000010110110000010110001000001010010110100111100110101001010110101110011110010100100110000011000111101111011011010000101100100111111000101111100010010110011101111011111100111011111001000100001111001011100100011101110011010101111100010000110010011000010100010011010000110111100001111111111011101011000000111100110101011001001101011010001101111010101001001101111000100010000101000000010010101101010001101101100100000111010000110100011111100000010000001101111011110001100000010110010001001111000010110001101111011000000000");
}

// {'seed': 27, 'version': 10, 'quality': 2}
// Download error: START
// Download error: DONE

#[test]
fn structure_codewords_seed_27() {
    const VERSION: crate::comptime::qrcode::version::Version =
        crate::comptime::qrcode::version::Version::V10;
    const QUALITY: crate::comptime::qrcode::ecl::ECL = crate::comptime::qrcode::ecl::ECL::Q;

    let data_codewords = &[
        245, 73, 50, 18, 16, 65, 211, 138, 85, 64, 95, 213, 208, 103, 46, 63, 232, 61, 125, 210,
        18, 186, 234, 163, 146, 167, 20, 156, 214, 108, 198, 106, 189, 13, 168, 112, 254, 89, 3,
        216, 249, 208, 164, 122, 64, 35, 156, 240, 86, 156, 166, 58, 180, 208, 134, 36, 25, 249,
        100, 192, 77, 233, 191, 203, 238, 9, 198, 174, 135, 21, 43, 117, 181, 38, 67, 138, 13, 44,
        249, 247, 166, 3, 115, 199, 38, 88, 227, 146, 147, 106, 16, 78, 112, 223, 208, 119, 90, 3,
        92, 40, 200, 67, 45, 123, 91, 224, 132, 15, 132, 63, 43, 53, 195, 188, 125, 142, 23, 85,
        86, 195, 14, 100, 87, 162, 29, 77, 65, 203, 142, 178, 176, 21, 76, 116, 118, 72, 131, 30,
        149, 5, 232, 72, 55, 226, 197, 122, 141, 138, 229, 247, 175, 30, 0, 151,
    ]
    .to_vec();
    let error_codewords = crate::comptime::qrcode::hardcode::get_polynomial(VERSION, QUALITY);

    let structure = crate::comptime::qrcode::polynomials::structure(
        &data_codewords,
        &error_codewords,
        QUALITY,
        VERSION,
    );

    let max = VERSION.max_bytes();
    let message = &structure[..data_codewords.len()];
    let errors = &structure[data_codewords.len()..max];
    assert_eq!(message.len(), 154);
    assert_eq!(
        message,
        [
            245, 210, 3, 249, 13, 119, 125, 118, 73, 18, 216, 100, 44, 90, 142, 72, 50, 186, 249,
            192, 249, 3, 23, 131, 18, 234, 208, 77, 247, 92, 85, 30, 16, 163, 164, 233, 166, 40,
            86, 149, 65, 146, 122, 191, 3, 200, 195, 5, 211, 167, 64, 203, 115, 67, 14, 232, 138,
            20, 35, 238, 199, 45, 100, 72, 85, 156, 156, 9, 38, 123, 87, 55, 64, 214, 240, 198, 88,
            91, 162, 226, 95, 108, 86, 174, 227, 224, 29, 197, 213, 198, 156, 135, 146, 132, 77,
            122, 208, 106, 166, 21, 147, 15, 65, 141, 103, 189, 58, 43, 106, 132, 203, 138, 46, 13,
            180, 117, 16, 63, 142, 229, 63, 168, 208, 181, 78, 43, 178, 247, 232, 112, 134, 38,
            112, 53, 176, 175, 61, 254, 36, 67, 223, 195, 21, 30, 125, 89, 25, 138, 208, 188, 76,
            0, 116, 151
        ]
        .to_vec()
    );

    assert_eq!(errors.len(), 192);
    assert_eq!(
        errors,
        [
            166, 190, 42, 17, 106, 180, 81, 84, 163, 36, 57, 172, 215, 224, 161, 223, 219, 74, 147,
            240, 91, 28, 14, 127, 146, 36, 211, 64, 246, 42, 17, 150, 72, 236, 225, 250, 46, 245,
            30, 57, 145, 91, 172, 166, 4, 18, 21, 1, 115, 26, 163, 248, 200, 75, 149, 43, 36, 172,
            251, 114, 247, 245, 85, 119, 65, 95, 108, 229, 163, 41, 159, 62, 4, 100, 234, 207, 61,
            185, 126, 225, 137, 55, 142, 43, 163, 130, 26, 19, 133, 184, 50, 239, 216, 5, 188, 67,
            161, 185, 184, 209, 42, 200, 9, 232, 235, 12, 210, 133, 42, 66, 50, 92, 122, 66, 109,
            39, 172, 232, 203, 67, 130, 80, 218, 224, 51, 73, 23, 5, 38, 190, 0, 244, 219, 48, 78,
            31, 155, 110, 20, 29, 159, 22, 254, 144, 91, 126, 156, 224, 53, 189, 98, 116, 150, 212,
            94, 101, 160, 71, 84, 232, 238, 96, 0, 133, 236, 216, 45, 188, 98, 193, 90, 37, 177,
            216, 181, 77, 163, 23, 61, 77, 170, 12, 90, 29, 99, 125, 186, 179, 176, 186, 0, 78
        ]
        .to_vec()
    );
}

// {'seed': 31, 'version': 7, 'quality': 3}
// Download error: START
// Download error: DONE

#[test]
fn structure_codewords_seed_31() {
    const VERSION: crate::comptime::qrcode::version::Version =
        crate::comptime::qrcode::version::Version::V07;
    const QUALITY: crate::comptime::qrcode::ecl::ECL = crate::comptime::qrcode::ecl::ECL::H;

    let data_codewords = &[
        28, 195, 100, 36, 175, 11, 35, 243, 28, 137, 59, 182, 193, 35, 37, 251, 189, 8, 169, 15,
        34, 59, 137, 187, 114, 134, 105, 52, 151, 23, 30, 5, 197, 240, 227, 103, 87, 50, 52, 84,
        100, 93, 152, 244, 63, 53, 185, 55, 106, 149, 169, 140, 15, 244, 13, 214, 46, 91, 34, 52,
        37, 214, 203, 253, 96, 7,
    ]
    .to_vec();

    let error_codewords = crate::comptime::qrcode::hardcode::get_polynomial(VERSION, QUALITY);

    let structure = crate::comptime::qrcode::polynomials::structure(
        &data_codewords,
        &error_codewords,
        QUALITY,
        VERSION,
    );

    let max = VERSION.max_bytes();
    let message = &structure[..data_codewords.len()];
    let errors = &structure[data_codewords.len()..max];

    assert_eq!(message.len(), 66);
    assert_eq!(
        message,
        [
            28, 35, 105, 84, 15, 195, 37, 52, 100, 244, 100, 251, 151, 93, 13, 36, 189, 23, 152,
            214, 175, 8, 30, 244, 46, 11, 169, 5, 63, 91, 35, 15, 197, 53, 34, 243, 34, 240, 185,
            52, 28, 59, 227, 55, 37, 137, 137, 103, 106, 214, 59, 187, 87, 149, 203, 182, 114, 50,
            169, 253, 193, 134, 52, 140, 96, 7
        ]
        .to_vec()
    );

    assert_eq!(errors.len(), 130);
    assert_eq!(
        errors,
        [
            68, 246, 227, 150, 102, 150, 74, 149, 57, 126, 68, 169, 27, 171, 43, 205, 24, 71, 98,
            193, 197, 210, 26, 1, 226, 78, 247, 175, 253, 205, 104, 165, 128, 9, 230, 100, 59, 66,
            53, 21, 177, 102, 125, 48, 138, 0, 186, 249, 53, 115, 185, 144, 124, 103, 110, 7, 234,
            168, 164, 76, 178, 202, 73, 222, 151, 106, 247, 42, 174, 81, 110, 84, 254, 204, 3, 170,
            191, 173, 38, 202, 101, 166, 130, 46, 90, 222, 28, 25, 48, 33, 45, 140, 166, 16, 17,
            74, 190, 194, 221, 95, 31, 219, 84, 191, 132, 75, 81, 128, 212, 153, 3, 72, 200, 114,
            178, 126, 34, 40, 38, 11, 216, 159, 177, 42, 114, 208, 0, 97, 99, 248
        ]
        .to_vec()
    );
}

// {'seed': 51, 'version': 16, 'quality': 1}
// Download error: START
// Download error: DONE

#[test]
fn structure_codewords_seed_51() {
    const VERSION: crate::comptime::qrcode::version::Version =
        crate::comptime::qrcode::version::Version::V16;
    const QUALITY: crate::comptime::qrcode::ecl::ECL = crate::comptime::qrcode::ecl::ECL::M;

    let data_codewords = &[
        62, 210, 254, 59, 213, 141, 65, 237, 222, 101, 140, 247, 118, 187, 178, 140, 134, 161, 87,
        150, 169, 92, 99, 71, 103, 178, 251, 1, 218, 29, 7, 137, 66, 8, 122, 173, 208, 241, 45, 88,
        157, 76, 94, 114, 201, 203, 237, 219, 195, 166, 49, 195, 194, 7, 112, 244, 214, 43, 120, 3,
        232, 15, 252, 27, 210, 201, 211, 188, 209, 202, 151, 27, 44, 140, 252, 242, 118, 238, 4,
        174, 111, 238, 106, 233, 51, 250, 245, 94, 28, 124, 231, 248, 140, 199, 45, 150, 91, 253,
        115, 126, 30, 132, 195, 112, 166, 2, 251, 111, 212, 138, 201, 10, 121, 238, 131, 164, 217,
        138, 25, 134, 207, 208, 78, 236, 136, 212, 117, 214, 175, 7, 83, 107, 135, 22, 225, 149,
        220, 148, 104, 143, 229, 219, 71, 57, 231, 186, 120, 150, 123, 54, 207, 250, 138, 13, 82,
        119, 78, 137, 170, 152, 113, 251, 51, 141, 90, 226, 182, 75, 64, 123, 199, 37, 140, 163,
        226, 108, 226, 200, 45, 28, 76, 118, 224, 245, 188, 2, 171, 169, 40, 173, 107, 185, 19,
        142, 129, 128, 192, 245, 242, 72, 148, 180, 151, 171, 112, 186, 114, 122, 97, 4, 163, 198,
        176, 71, 35, 200, 175, 252, 249, 198, 102, 103, 231, 219, 163, 98, 81, 81, 102, 146, 252,
        167, 249, 175, 84, 141, 117, 104, 130, 190, 151, 83, 122, 155, 31, 1, 240, 33, 100, 222,
        242, 55, 206, 105, 66, 13, 100, 195, 143, 84, 210, 12, 135, 70, 8, 238, 82, 193, 237, 0,
        74, 244, 115, 18, 153, 123, 186, 136, 225, 200, 141, 36, 89, 211, 232, 70, 20, 188, 124,
        237, 128, 151, 214, 0, 61, 170, 197, 111, 47, 186, 205, 200, 230, 27, 225, 145, 240, 164,
        185, 237, 2, 157, 135, 157, 186, 196, 212, 40, 253, 220, 197, 148, 122, 162, 34, 39, 210,
        133, 137, 104, 106, 52, 125, 240, 13, 216, 117, 117, 103, 19, 141, 251, 11, 30, 75, 129,
        19, 144, 49, 218, 37, 167, 16, 154, 216, 131, 51, 250, 194, 209, 112, 91, 57, 203, 206,
        242, 214, 124, 59, 21, 171, 17, 123, 247, 139, 53, 43, 205, 98, 49, 140, 57, 28, 41, 229,
        208, 143, 23, 217, 120, 187, 207, 120, 134, 4, 2, 48, 4, 84, 212, 238, 76, 76, 21, 41, 134,
        221, 200, 186, 77, 24, 34, 2, 47, 15, 121, 186, 199, 83, 21, 24, 194, 219, 174, 204, 142,
        189, 98, 73, 8, 162, 129, 211, 105, 83, 173, 150, 15, 113, 42, 11, 4, 77, 124, 83, 15, 181,
        136, 10, 111, 163, 84, 227,
    ]
    .to_vec();
    let error_codewords = crate::comptime::qrcode::hardcode::get_polynomial(VERSION, QUALITY);

    let structure = crate::comptime::qrcode::polynomials::structure(
        &data_codewords,
        &error_codewords,
        QUALITY,
        VERSION,
    );
    let max = VERSION.max_bytes();
    let message = &structure[..data_codewords.len()];
    let errors = &structure[data_codewords.len()..max];
    assert_eq!(message.len(), 453);
    assert_eq!(
        message,
        [
            62, 203, 231, 149, 76, 98, 74, 196, 91, 200, 210, 237, 248, 220, 118, 81, 244, 212, 57,
            186, 254, 219, 140, 148, 224, 81, 115, 40, 203, 77, 59, 195, 199, 104, 245, 102, 18,
            253, 206, 24, 213, 166, 45, 143, 188, 146, 153, 220, 242, 34, 141, 49, 150, 229, 2,
            252, 123, 197, 214, 2, 65, 195, 91, 219, 171, 167, 186, 148, 124, 47, 237, 194, 253,
            71, 169, 249, 136, 122, 59, 15, 222, 7, 115, 57, 40, 175, 225, 162, 21, 121, 101, 112,
            126, 231, 173, 84, 200, 34, 171, 186, 140, 244, 30, 186, 107, 141, 141, 39, 17, 199,
            247, 214, 132, 120, 185, 117, 36, 210, 123, 83, 118, 43, 195, 150, 19, 104, 89, 133,
            247, 21, 187, 120, 112, 123, 142, 130, 211, 137, 139, 24, 178, 3, 166, 54, 129, 190,
            232, 104, 53, 194, 140, 232, 2, 207, 128, 151, 70, 106, 43, 219, 134, 15, 251, 250,
            192, 83, 20, 52, 205, 174, 161, 252, 111, 138, 245, 122, 188, 125, 98, 204, 87, 27,
            212, 13, 242, 155, 124, 240, 49, 142, 150, 210, 138, 82, 72, 31, 237, 13, 140, 189,
            169, 201, 201, 119, 148, 1, 128, 216, 57, 98, 92, 211, 10, 78, 180, 240, 151, 117, 28,
            73, 99, 188, 121, 137, 151, 33, 214, 117, 41, 8, 71, 209, 238, 170, 171, 100, 0, 103,
            229, 162, 103, 202, 131, 152, 112, 222, 61, 19, 208, 129, 178, 151, 164, 113, 186, 242,
            170, 141, 143, 211, 251, 27, 217, 251, 114, 55, 197, 251, 23, 105, 1, 44, 138, 51, 122,
            206, 111, 11, 217, 83, 218, 140, 25, 141, 97, 105, 47, 30, 120, 173, 29, 252, 134, 90,
            4, 66, 186, 75, 187, 150, 7, 242, 207, 226, 163, 13, 205, 129, 207, 15, 137, 118, 208,
            182, 198, 100, 200, 19, 120, 113, 66, 238, 78, 75, 176, 195, 230, 144, 134, 42, 8, 4,
            236, 64, 71, 143, 27, 49, 4, 11, 122, 174, 136, 123, 35, 84, 225, 218, 2, 4, 173, 111,
            212, 199, 200, 210, 145, 37, 48, 77, 208, 238, 117, 37, 175, 12, 240, 167, 4, 124, 241,
            106, 214, 140, 252, 135, 164, 16, 84, 83, 45, 233, 175, 163, 249, 70, 185, 154, 212,
            15, 88, 51, 7, 226, 198, 8, 237, 216, 238, 181, 157, 250, 83, 108, 102, 238, 2, 131,
            76, 136, 76, 245, 107, 226, 103, 82, 157, 51, 76, 10, 94, 94, 135, 200, 231, 193, 135,
            250, 21, 111, 114, 28, 22, 45, 219, 237, 157, 194, 41, 163, 201, 124, 225, 28, 163, 0,
            186, 209, 134, 84, 112, 221, 227
        ]
        .to_vec()
    );

    assert_eq!(errors.len(), 280);
    assert_eq!(
        errors,
        [
            206, 181, 144, 198, 59, 152, 23, 133, 246, 151, 191, 8, 145, 23, 61, 92, 8, 228, 14,
            72, 189, 36, 100, 176, 95, 7, 206, 184, 41, 5, 112, 9, 117, 6, 179, 114, 5, 161, 87,
            152, 114, 5, 234, 189, 154, 165, 20, 24, 47, 35, 106, 44, 128, 95, 202, 101, 65, 70,
            153, 118, 117, 34, 83, 37, 251, 34, 1, 203, 237, 210, 25, 19, 112, 195, 219, 199, 176,
            136, 116, 227, 225, 82, 61, 70, 60, 91, 50, 175, 243, 220, 50, 162, 5, 182, 68, 198,
            68, 242, 148, 52, 141, 251, 171, 9, 9, 150, 184, 252, 140, 122, 237, 79, 154, 132, 35,
            250, 33, 14, 36, 35, 37, 177, 245, 185, 198, 8, 213, 7, 23, 97, 90, 159, 19, 103, 48,
            69, 179, 199, 162, 135, 10, 24, 69, 157, 250, 54, 29, 173, 170, 65, 182, 211, 70, 141,
            240, 92, 222, 167, 107, 183, 7, 176, 25, 40, 162, 122, 104, 143, 214, 21, 93, 38, 205,
            74, 169, 120, 33, 9, 118, 158, 136, 186, 204, 130, 11, 182, 204, 126, 220, 90, 244,
            123, 21, 127, 246, 37, 7, 169, 23, 202, 177, 154, 33, 149, 141, 164, 8, 195, 221, 111,
            134, 167, 222, 171, 96, 225, 225, 78, 235, 229, 103, 185, 150, 145, 40, 80, 59, 218,
            32, 110, 47, 199, 79, 10, 244, 204, 252, 88, 42, 107, 244, 231, 11, 81, 227, 203, 189,
            182, 125, 108, 109, 209, 44, 162, 19, 94, 13, 196, 17, 152, 235, 20, 154, 34, 106, 197,
            0, 230, 183, 244, 209, 205, 61, 207, 142, 191, 42, 163, 143, 142
        ]
        .to_vec()
    );
}

// {'seed': 57, 'version': 3, 'quality': 2}
// Download error: START
// Download error: DONE

#[test]
fn structure_codewords_seed_57() {
    const VERSION: crate::comptime::qrcode::version::Version =
        crate::comptime::qrcode::version::Version::V03;
    const QUALITY: crate::comptime::qrcode::ecl::ECL = crate::comptime::qrcode::ecl::ECL::Q;

    let data_codewords = &[
        150, 154, 4, 57, 131, 82, 250, 122, 74, 251, 249, 111, 162, 213, 226, 96, 5, 221, 188, 42,
        132, 233, 154, 95, 96, 101, 250, 27, 167, 225, 185, 149, 174, 124,
    ]
    .to_vec();
    let error_codewords = crate::comptime::qrcode::hardcode::get_polynomial(VERSION, QUALITY);

    let structure = crate::comptime::qrcode::polynomials::structure(
        &data_codewords,
        &error_codewords,
        QUALITY,
        VERSION,
    );

    let max = VERSION.max_bytes();

    let message = &structure[..data_codewords.len()];
    let errors = &structure[data_codewords.len()..max];
    assert_eq!(message.len(), 34);
    assert_eq!(
        message,
        [
            150, 221, 154, 188, 4, 42, 57, 132, 131, 233, 82, 154, 250, 95, 122, 96, 74, 101, 251,
            250, 249, 27, 111, 167, 162, 225, 213, 185, 226, 149, 96, 174, 5, 124
        ]
        .to_vec()
    );

    assert_eq!(errors.len(), 36);
    assert_eq!(
        errors,
        [
            206, 0, 168, 196, 17, 49, 126, 61, 120, 99, 183, 113, 218, 138, 137, 196, 118, 150,
            204, 134, 53, 174, 77, 202, 129, 112, 167, 38, 240, 12, 36, 137, 50, 29, 32, 145
        ]
        .to_vec()
    );
}

#[test]
fn placement() {
    let version = crate::comptime::qrcode::version::Version::V02;
    // let quality = crate::ecl::ECL::M;

    let mut bs = crate::comptime::qrcode::bitstring::BitString::new();

    for i in 0..44 {
        bs = crate::comptime::qrcode::bitstring::BitString::push_u8(bs, (i * 11) as u8);
    }

    let mut mat = [[false; 25]; 25];
    let mat_full = crate::comptime::qrcode::default::non_available_matrix_from_version(version);

    mat = crate::comptime::qrcode::placement::test_place_on_matrix_data(mat, bs, &mat_full);

    let mut results = Vec::new();
    for i in 0..44 {
        let tmp = (i * 11) as u8;
        results.push((tmp >> 7 & 1) != 0);
        results.push((tmp >> 6 & 1) != 0);
        results.push((tmp >> 5 & 1) != 0);
        results.push((tmp >> 4 & 1) != 0);
        results.push((tmp >> 3 & 1) != 0);
        results.push((tmp >> 2 & 1) != 0);
        results.push((tmp >> 1 & 1) != 0);
        results.push((tmp >> 0 & 1) != 0);
    }

    // Manual testing
    {
        let mut start = 0;
        for i in 0..16 {
            assert_eq!(results[start + i * 2], mat[24 - i][24]);
            assert_eq!(results[start + i * 2 + 1], mat[24 - i][23]);
        }

        start += 16 * 2;
        for i in 0..16 {
            assert_eq!(results[start + i * 2], mat[9 + i][22]);
            assert_eq!(results[start + i * 2 + 1], mat[9 + i][21]);
        }

        start += 16 * 2;
        for i in 0..4 {
            assert_eq!(results[start + i * 2], mat[24 - i][20]);
            assert_eq!(results[start + i * 2 + 1], mat[24 - i][19]);
        }

        start += 4 * 2;
        for i in 0..7 {
            assert_eq!(results[start + i * 2], mat[24 - 9 - i][20]);
            assert_eq!(results[start + i * 2 + 1], mat[24 - 9 - i][19]);
        }

        start += 7 * 2;
        for i in 0..7 {
            assert_eq!(results[start + i * 2], mat[9 + i][18]);
            assert_eq!(results[start + i * 2 + 1], mat[9 + i][17]);
        }

        start += 7 * 2;
        for i in 0..4 {
            assert_eq!(results[start + i * 2], mat[21 + i][18]);
            assert_eq!(results[start + i * 2 + 1], mat[21 + i][17]);
        }

        start += 4 * 2;
        for i in 0..4 {
            assert_eq!(results[start + i * 2], mat[24 - i][16]);
            assert_eq!(results[start + i * 2 + 1], mat[24 - i][15]);
        }

        start += 4 * 2;
        for i in 0..5 {
            assert_eq!(results[start + i], mat[20 - i][15]);
        }

        start += 5 * 1;
        for i in 0..9 {
            assert_eq!(results[start + i * 2], mat[15 - i][16]);
            assert_eq!(results[start + i * 2 + 1], mat[15 - i][15]);
        }

        start += 9 * 2;
        for i in 0..6 {
            assert_eq!(results[start + i * 2], mat[5 - i][16]);
            assert_eq!(results[start + i * 2 + 1], mat[5 - i][15]);
        }

        start += 6 * 2;
        for i in 0..6 {
            assert_eq!(results[start + i * 2], mat[i][14]);
            assert_eq!(results[start + i * 2 + 1], mat[i][13]);
        }

        start += 6 * 2;
        for i in 0..18 {
            assert_eq!(results[start + i * 2], mat[7 + i][14]);
            assert_eq!(results[start + i * 2 + 1], mat[7 + i][13]);
        }

        start += 18 * 2;
        for i in 0..18 {
            assert_eq!(results[start + i * 2], mat[24 - i][12]);
            assert_eq!(results[start + i * 2 + 1], mat[24 - i][11]);
        }

        start += 18 * 2;
        for i in 0..6 {
            assert_eq!(results[start + i * 2], mat[5 - i][12]);
            assert_eq!(results[start + i * 2 + 1], mat[5 - i][11]);
        }

        start += 6 * 2;
        for i in 0..6 {
            assert_eq!(results[start + i * 2], mat[i][10]);
            assert_eq!(results[start + i * 2 + 1], mat[i][9]);
        }

        start += 6 * 2;
        for i in 0..18 {
            assert_eq!(results[start + i * 2], mat[7 + i][10]);
            assert_eq!(results[start + i * 2 + 1], mat[7 + i][9]);
        }

        start += 18 * 2;
        for i in 0..8 {
            assert_eq!(results[start + i * 2], mat[16 - i][8]);
            assert_eq!(results[start + i * 2 + 1], mat[16 - i][7]);
        }

        start += 8 * 2;
        for i in 0..8 {
            assert_eq!(results[start + i * 2], mat[9 + i][5]);
            assert_eq!(results[start + i * 2 + 1], mat[9 + i][4]);
        }

        start += 8 * 2;
        for i in 0..8 {
            assert_eq!(results[start + i * 2], mat[16 - i][3]);
            assert_eq!(results[start + i * 2 + 1], mat[16 - i][2]);
        }

        start += 8 * 2;
        for i in 0..4 {
            assert_eq!(results[start + i * 2], mat[9 + i][1]);
            assert_eq!(results[start + i * 2 + 1], mat[9 + i][0]);
        }

        assert_eq!(results[results.len() - 1], mat[13][1]);
        assert_eq!(false, mat[14][0]);
        assert_eq!(false, mat[14][1]);
        assert_eq!(false, mat[15][0]);
        assert_eq!(false, mat[15][1]);
        assert_eq!(false, mat[18][0]);
        assert_eq!(false, mat[18][1]);
    }
}
