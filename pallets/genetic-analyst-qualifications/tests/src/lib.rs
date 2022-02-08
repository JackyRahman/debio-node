mod mock;

#[cfg(test)]
mod tests {
	use crate::mock::*;
	
	use genetic_analysts::GeneticAnalystInfo;
	use genetic_analyst_qualifications::{
		Error, 
		GeneticAnalystQualification, GeneticAnalystQualificationInfo,
		GeneticAnalystCertification, GeneticAnalystExperience
	};
	
	use frame_support::{
		assert_noop, assert_ok,
		sp_runtime::traits::{Hash, Keccak256},
	};
	
	#[test]
	fn create_genetic_analyst_qualification_works() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(GeneticAnalysts::register_genetic_analyst(
				Origin::signed(1),
				GeneticAnalystInfo {
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				}
			));
			
			assert_ok!(GeneticAnalystQualifications::create_qualification(
				Origin::signed(1),
				GeneticAnalystQualificationInfo {
					experience: vec![
						GeneticAnalystExperience {
							title: "DeBio title".as_bytes().to_vec(),
						}
					],
					certification: Some(
						vec![
							GeneticAnalystCertification {
								title: "DeBio title".as_bytes().to_vec(),
								issuer: "DeBio issuer".as_bytes().to_vec(),
								month: "DeBio month".as_bytes().to_vec(),
								year: "DeBio year".as_bytes().to_vec(),
								description: "DeBio description".as_bytes().to_vec(),
								supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
							}
						]
					),
				}
			));
	
			let genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();
	
			assert_eq!(
				GeneticAnalystQualifications::qualification_by_id(genetic_analyst.qualifications[0]),
				Some(
					GeneticAnalystQualification {
						id: genetic_analyst.qualifications[0],
						owner_id: 1,
						info: GeneticAnalystQualificationInfo {
							experience: vec![
								GeneticAnalystExperience {
									title: "DeBio title".as_bytes().to_vec(),
								}
							],
							certification: Some(
								vec![
									GeneticAnalystCertification {
										title: "DeBio title".as_bytes().to_vec(),
										issuer: "DeBio issuer".as_bytes().to_vec(),
										month: "DeBio month".as_bytes().to_vec(),
										year: "DeBio year".as_bytes().to_vec(),
										description: "DeBio description".as_bytes().to_vec(),
										supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
									}
								]
							),
						}
					}
				)
			);
	
			assert_eq!(
				GeneticAnalystQualifications::qualification_count_by_owner(1),
				Some(1)
			);
		})
	}
	
	#[test]
	fn update_genetic_analyst_qualification_works() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(GeneticAnalysts::register_genetic_analyst(
				Origin::signed(1),
				GeneticAnalystInfo {
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				}
			));
			
			assert_ok!(GeneticAnalystQualifications::create_qualification(
				Origin::signed(1),
				GeneticAnalystQualificationInfo {
					experience: vec![
						GeneticAnalystExperience {
							title: "DeBio title".as_bytes().to_vec(),
						}
					],
					certification: Some(
						vec![
							GeneticAnalystCertification {
								title: "DeBio title".as_bytes().to_vec(),
								issuer: "DeBio issuer".as_bytes().to_vec(),
								month: "DeBio month".as_bytes().to_vec(),
								year: "DeBio year".as_bytes().to_vec(),
								description: "DeBio description".as_bytes().to_vec(),
								supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
							}
						]
					),
				}
			));
	
			let genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();
			
			assert_ok!(GeneticAnalystQualifications::update_qualification(
				Origin::signed(1),
				genetic_analyst.qualifications[0],
				GeneticAnalystQualificationInfo {
					experience: vec![
						GeneticAnalystExperience {
							title: "DeBio title 2".as_bytes().to_vec(),
						}
					],
					certification: Some(
						vec![
							GeneticAnalystCertification {
								title: "DeBio title 2".as_bytes().to_vec(),
								issuer: "DeBio issuer 2".as_bytes().to_vec(),
								month: "DeBio month 2".as_bytes().to_vec(),
								year: "DeBio year 2".as_bytes().to_vec(),
								description: "DeBio description 2".as_bytes().to_vec(),
								supporting_document: Some("DeBio Profile Image uwu 2".as_bytes().to_vec()),
							}
						]
					),
				}
			));
	
			assert_eq!(
				GeneticAnalystQualifications::qualification_by_id(genetic_analyst.qualifications[0]),
				Some(
					GeneticAnalystQualification {
						id: genetic_analyst.qualifications[0],
						owner_id: 1,
						info: GeneticAnalystQualificationInfo {
							experience: vec![
								GeneticAnalystExperience {
									title: "DeBio title 2".as_bytes().to_vec(),
								}
							],
							certification: Some(
								vec![
									GeneticAnalystCertification {
										title: "DeBio title 2".as_bytes().to_vec(),
										issuer: "DeBio issuer 2".as_bytes().to_vec(),
										month: "DeBio month 2".as_bytes().to_vec(),
										year: "DeBio year 2".as_bytes().to_vec(),
										description: "DeBio description 2".as_bytes().to_vec(),
										supporting_document: Some("DeBio Profile Image uwu 2".as_bytes().to_vec()),
									}
								]
							),
						}
					}
				)
			);
	
			assert_eq!(
				GeneticAnalystQualifications::qualification_count_by_owner(1),
				Some(1)
			);
		})
	}
	
	#[test]
	fn delete_genetic_analyst_qualification_works() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(GeneticAnalysts::register_genetic_analyst(
				Origin::signed(1),
				GeneticAnalystInfo {
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				}
			));
			
			assert_ok!(GeneticAnalystQualifications::create_qualification(
				Origin::signed(1),
				GeneticAnalystQualificationInfo {
					experience: vec![
						GeneticAnalystExperience {
							title: "DeBio title".as_bytes().to_vec(),
						}
					],
					certification: Some(
						vec![
							GeneticAnalystCertification {
								title: "DeBio title".as_bytes().to_vec(),
								issuer: "DeBio issuer".as_bytes().to_vec(),
								month: "DeBio month".as_bytes().to_vec(),
								year: "DeBio year".as_bytes().to_vec(),
								description: "DeBio description".as_bytes().to_vec(),
								supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
							}
						]
					),
				}
			));
	
			let genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();
			
			assert_ok!(GeneticAnalystQualifications::delete_qualification(
				Origin::signed(1),
				genetic_analyst.qualifications[0]
			));
	
			assert_eq!(
				GeneticAnalystQualifications::qualification_count_by_owner(1),
				Some(0)
			);
		})
	}
	
	#[test]
	fn not_allowed_to_create_genetic_analyst_qualification() {
		ExternalityBuilder::build().execute_with(|| {		
			assert_noop!(
				GeneticAnalystQualifications::create_qualification(
					Origin::signed(1),
					GeneticAnalystQualificationInfo {
						experience: vec![
							GeneticAnalystExperience {
								title: "DeBio title".as_bytes().to_vec(),
							}
						],
						certification: Some(
							vec![
								GeneticAnalystCertification {
									title: "DeBio title".as_bytes().to_vec(),
									issuer: "DeBio issuer".as_bytes().to_vec(),
									month: "DeBio month".as_bytes().to_vec(),
									year: "DeBio year".as_bytes().to_vec(),
									description: "DeBio description".as_bytes().to_vec(),
									supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
								}
							]
						),
					}
				),
				Error::<Test>::NotAllowedToCreate
			);
		})
	}
	
	#[test]
	fn update_genetic_analyst_qualification_does_not_exist() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(GeneticAnalysts::register_genetic_analyst(
				Origin::signed(1),
				GeneticAnalystInfo {
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				}
			));
			
			assert_noop!(
				GeneticAnalystQualifications::update_qualification(
					Origin::signed(1),
					Keccak256::hash(
						"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
					),
					GeneticAnalystQualificationInfo {
						experience: vec![
							GeneticAnalystExperience {
								title: "DeBio title 2".as_bytes().to_vec(),
							}
						],
						certification: Some(
							vec![
								GeneticAnalystCertification {
									title: "DeBio title 2".as_bytes().to_vec(),
									issuer: "DeBio issuer 2".as_bytes().to_vec(),
									month: "DeBio month 2".as_bytes().to_vec(),
									year: "DeBio year 2".as_bytes().to_vec(),
									description: "DeBio description 2".as_bytes().to_vec(),
									supporting_document: Some("DeBio Profile Image uwu 2".as_bytes().to_vec()),
								}
							]
						),
					}
				),
				Error::<Test>::GeneticAnalystQualificationDoesNotExist
			);
		})
	}
	
	#[test]
	fn update_genetic_analyst_qualification_not_owner() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(GeneticAnalysts::register_genetic_analyst(
				Origin::signed(1),
				GeneticAnalystInfo {
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				}
			));
			
			assert_ok!(GeneticAnalystQualifications::create_qualification(
				Origin::signed(1),
				GeneticAnalystQualificationInfo {
					experience: vec![
						GeneticAnalystExperience {
							title: "DeBio title".as_bytes().to_vec(),
						}
					],
					certification: Some(
						vec![
							GeneticAnalystCertification {
								title: "DeBio title".as_bytes().to_vec(),
								issuer: "DeBio issuer".as_bytes().to_vec(),
								month: "DeBio month".as_bytes().to_vec(),
								year: "DeBio year".as_bytes().to_vec(),
								description: "DeBio description".as_bytes().to_vec(),
								supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
							}
						]
					),
				}
			));
	
			let genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();
			
			assert_noop!(
				GeneticAnalystQualifications::update_qualification(
					Origin::signed(2),
					genetic_analyst.qualifications[0],
					GeneticAnalystQualificationInfo {
						experience: vec![
							GeneticAnalystExperience {
								title: "DeBio title 2".as_bytes().to_vec(),
							}
						],
						certification: Some(
							vec![
								GeneticAnalystCertification {
									title: "DeBio title 2".as_bytes().to_vec(),
									issuer: "DeBio issuer 2".as_bytes().to_vec(),
									month: "DeBio month 2".as_bytes().to_vec(),
									year: "DeBio year 2".as_bytes().to_vec(),
									description: "DeBio description 2".as_bytes().to_vec(),
									supporting_document: Some("DeBio Profile Image uwu 2".as_bytes().to_vec()),
								}
							]
						),
					}
				),
				Error::<Test>::NotGeneticAnalystQualificationOwner
			);
		})
	}
	
	#[test]
	fn delete_genetic_analyst_qualification_does_not_exist() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(GeneticAnalysts::register_genetic_analyst(
				Origin::signed(1),
				GeneticAnalystInfo {
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				}
			));
			
			assert_noop!(
				GeneticAnalystQualifications::delete_qualification(
					Origin::signed(1),
					Keccak256::hash(
						"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
					)
				),
				Error::<Test>::GeneticAnalystQualificationDoesNotExist
			);
		})
	}
	
	#[test]
	fn delete_genetic_analyst_qualification_not_owner() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(GeneticAnalysts::register_genetic_analyst(
				Origin::signed(1),
				GeneticAnalystInfo {
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				}
			));
			
			assert_ok!(GeneticAnalystQualifications::create_qualification(
				Origin::signed(1),
				GeneticAnalystQualificationInfo {
					experience: vec![
						GeneticAnalystExperience {
							title: "DeBio title".as_bytes().to_vec(),
						}
					],
					certification: Some(
						vec![
							GeneticAnalystCertification {
								title: "DeBio title".as_bytes().to_vec(),
								issuer: "DeBio issuer".as_bytes().to_vec(),
								month: "DeBio month".as_bytes().to_vec(),
								year: "DeBio year".as_bytes().to_vec(),
								description: "DeBio description".as_bytes().to_vec(),
								supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
							}
						]
					),
				}
			));
	
			let genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();
			
			assert_noop!(
				GeneticAnalystQualifications::delete_qualification(
					Origin::signed(2),
					genetic_analyst.qualifications[0]
				),
				Error::<Test>::NotGeneticAnalystQualificationOwner
			);
		})
	}
}