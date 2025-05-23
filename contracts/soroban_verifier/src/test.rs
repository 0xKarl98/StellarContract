// #![cfg(test)]
// extern crate std;

// use soroban_sdk::{Bytes, Env};

// use crate::{BN254Contract, BN254ContractClient};

// #[test]
// fn test_generate_g1_point() {
//     let env = Env::default();
//     let contract_id = env.register(BN254Contract, ());
//     let client = BN254ContractClient::new(&env, &contract_id);

//     let scalar_bytes = Bytes::from_array(
//         &env,
//         &[
//             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//             0, 0, 1,
//         ],
//     );

//     let point = client.generate_g1_point(&scalar_bytes);

//     assert!(point.len() > 0);

//     let result = client.scalar_mul_g1(&point, &scalar_bytes);

//     assert!(result.len() > 0);
// }

// #[test]
// fn test_scalar_mul_g1() {
//     let env = Env::default();
//     let contract_id = env.register(BN254Contract, ());
//     let client = BN254ContractClient::new(&env, &contract_id);

//     let scalar_one = Bytes::from_array(
//         &env,
//         &[
//             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//             0, 0, 1,
//         ],
//     );

//     let point = client.generate_g1_point(&scalar_one);

//     let scalar_two = Bytes::from_array(
//         &env,
//         &[
//             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//             0, 0, 2,
//         ],
//     );

//     let result = client.scalar_mul_g1(&point, &scalar_two);

//     assert!(result.len() > 0);

//     let point_plus_point = client.add_g1_points(&point, &point);
//     assert_eq!(result, point_plus_point);
// }

// #[test]
// fn test_add_g1_points() {
//     let env = Env::default();
//     let contract_id = env.register(BN254Contract, ());
//     let client = BN254ContractClient::new(&env, &contract_id);

//     let scalar_one = Bytes::from_array(
//         &env,
//         &[
//             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//             0, 0, 1,
//         ],
//     );

//     let scalar_two = Bytes::from_array(
//         &env,
//         &[
//             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//             0, 0, 2,
//         ],
//     );

//     let point1 = client.generate_g1_point(&scalar_one);
//     let point2 = client.generate_g1_point(&scalar_two);

//     let sum = client.add_g1_points(&point1, &point2);

//     assert!(sum.len() > 0);

//     let scalar_three = Bytes::from_array(
//         &env,
//         &[
//             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//             0, 0, 3,
//         ],
//     );
//     client.generate_g1_point(&scalar_three);
// }

// #[test]
// fn test_perform_pairing() {
//     let env = Env::default();
//     let contract_id = env.register(BN254Contract, ());
//     let client = BN254ContractClient::new(&env, &contract_id);

//     let scalar_one = Bytes::from_array(
//         &env,
//         &[
//             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//             0, 0, 1,
//         ],
//     );

//     let g1_point = client.generate_g1_point(&scalar_one);

//     let g2_point = Bytes::from_array(
//         &env,
//         &[
//             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//             0, 0, 0, 0, 0, 0, 0, 0, 0,
//         ],
//     );

//     client.perform_pairing(&g1_point, &g2_point);
// }

// #[test]
// fn test_combined_operations() {
//     let env = Env::default();
//     let contract_id = env.register(BN254Contract, ());
//     let client = BN254ContractClient::new(&env, &contract_id);

//     let scalar_one = Bytes::from_array(
//         &env,
//         &[
//             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//             0, 0, 1,
//         ],
//     );

//     let scalar_two = Bytes::from_array(
//         &env,
//         &[
//             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//             0, 0, 2,
//         ],
//     );

//     let point1 = client.generate_g1_point(&scalar_one);

//     let point2 = client.scalar_mul_g1(&point1, &scalar_two);

//     let sum = client.add_g1_points(&point1, &point2);

//     assert!(sum.len() > 0);

//     let scalar_three = Bytes::from_array(
//         &env,
//         &[
//             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//             0, 0, 3,
//         ],
//     );

//     let point3 = client.scalar_mul_g1(&point1, &scalar_three);
//     assert_eq!(sum, point3);
// }
