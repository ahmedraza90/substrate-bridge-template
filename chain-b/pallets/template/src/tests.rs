use crate::{mock::*, Event, FarmAuditHistory, FarmVersionCounter};
use frame_support::assert_ok;
use sp_runtime::BoundedVec;

#[test]
fn add_farm_audit_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let farm_id = 123u64;
        let data = BoundedVec::try_from(b"farm audit data".to_vec()).unwrap();
        let data_format = 0u8; // JSON format

        // Add first audit record
        assert_ok!(Template::store_farm_update(
            RuntimeOrigin::signed(1),
            farm_id,
            data.clone(),
            data_format
        ));

        // Check that version counter was incremented
        assert_eq!(FarmVersionCounter::<Test>::get(farm_id), 1);

        // Check that audit record was stored
        let stored_record = FarmAuditHistory::<Test>::get(farm_id, 1).unwrap();
        assert_eq!(stored_record.data, data);
        assert_eq!(stored_record.updated_by, 1);
        assert_eq!(stored_record.data_format, data_format);

        // Check event was emitted
        System::assert_last_event(
            Event::FarmDataStored {
                farm_id,
                version: 1,
                who: 1,
                timestamp: stored_record.updated_at,
            }
            .into(),
        );
    });
}

#[test]
fn add_multiple_farm_audits_increments_version() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let farm_id = 456u64;
        let data1 = BoundedVec::try_from(b"first audit".to_vec()).unwrap();
        let data2 = BoundedVec::try_from(b"second audit".to_vec()).unwrap();

        // Add first audit
        assert_ok!(Template::store_farm_update(
            RuntimeOrigin::signed(1),
            farm_id,
            data1.clone(),
            0
        ));
        assert_eq!(FarmVersionCounter::<Test>::get(farm_id), 1);

        // Add second audit
        assert_ok!(Template::store_farm_update(
            RuntimeOrigin::signed(2),
            farm_id,
            data2.clone(),
            1
        ));
        assert_eq!(FarmVersionCounter::<Test>::get(farm_id), 2);

        // Check both records exist
        let record1 = FarmAuditHistory::<Test>::get(farm_id, 1).unwrap();
        let record2 = FarmAuditHistory::<Test>::get(farm_id, 2).unwrap();

        assert_eq!(record1.data, data1);
        assert_eq!(record1.updated_by, 1);
        assert_eq!(record2.data, data2);
        assert_eq!(record2.updated_by, 2);
    });
}

#[test]
fn get_latest_version_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let farm_id = 789u64;

        // Initially, version should be 0 (default)
        let version = Template::get_latest_version(farm_id);
        assert_eq!(version, 0);

        // Add an audit record
        let data = BoundedVec::try_from(b"test data".to_vec()).unwrap();
        assert_ok!(Template::store_farm_update(
            RuntimeOrigin::signed(1),
            farm_id,
            data,
            0
        ));

        // Now version should be 1
        let version = Template::get_latest_version(farm_id);
        assert_eq!(version, 1);
    });
}

#[test]
fn different_farms_have_independent_versions() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let farm_id_1 = 100u64;
        let farm_id_2 = 200u64;
        let data = BoundedVec::try_from(b"audit data".to_vec()).unwrap();

        // Add audit to farm 1
        assert_ok!(Template::store_farm_update(
            RuntimeOrigin::signed(1),
            farm_id_1,
            data.clone(),
            0
        ));

        // Add two audits to farm 2
        assert_ok!(Template::store_farm_update(
            RuntimeOrigin::signed(1),
            farm_id_2,
            data.clone(),
            0
        ));
        assert_ok!(Template::store_farm_update(
            RuntimeOrigin::signed(1),
            farm_id_2,
            data.clone(),
            0
        ));

        // Check versions are independent
        assert_eq!(FarmVersionCounter::<Test>::get(farm_id_1), 1);
        assert_eq!(FarmVersionCounter::<Test>::get(farm_id_2), 2);

        // Check records exist for both farms
        assert!(FarmAuditHistory::<Test>::get(farm_id_1, 1).is_some());
        assert!(FarmAuditHistory::<Test>::get(farm_id_2, 1).is_some());
        assert!(FarmAuditHistory::<Test>::get(farm_id_2, 2).is_some());
        assert!(FarmAuditHistory::<Test>::get(farm_id_1, 2).is_none());
    });
}

#[test]
fn add_farm_audit_with_large_data() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let farm_id = 999u64;
        // Create data at the limit (8192 bytes)
        let large_data = vec![42u8; 8192];
        let bounded_data = BoundedVec::try_from(large_data.clone()).unwrap();

        assert_ok!(Template::store_farm_update(
            RuntimeOrigin::signed(1),
            farm_id,
            bounded_data.clone(),
            0
        ));

        let stored_record = FarmAuditHistory::<Test>::get(farm_id, 1).unwrap();
        assert_eq!(stored_record.data.len(), 8192);
        assert_eq!(stored_record.data.to_vec(), large_data);
    });
}
