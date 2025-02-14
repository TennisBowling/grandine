use std::collections::BTreeMap;

use helper_functions::misc;
use types::{
    phase0::primitives::{CommitteeIndex, Epoch, ValidatorIndex},
    preset::Preset,
};

use crate::misc::BeaconCommitteeSubscription;

type ValidatorCommitteeSubscriptions = BTreeMap<CommitteeIndex, BeaconCommitteeSubscription>;

#[derive(Default, Clone)]
pub struct BeaconCommitteeSubscriptions {
    subscriptions: BTreeMap<Epoch, BTreeMap<ValidatorIndex, ValidatorCommitteeSubscriptions>>,
}

impl BeaconCommitteeSubscriptions {
    pub fn discard_old_subscriptions(&mut self, epoch: Epoch) {
        self.subscriptions = self.subscriptions.split_off(&epoch);
    }

    pub fn all(&self) -> impl Iterator<Item = BeaconCommitteeSubscription> + '_ {
        self.subscriptions
            .values()
            .flat_map(BTreeMap::values)
            .flat_map(BTreeMap::values)
            .copied()
    }

    pub fn update<P: Preset>(
        &mut self,
        subscriptions: impl IntoIterator<Item = BeaconCommitteeSubscription>,
    ) {
        for subscription in subscriptions {
            let epoch = misc::compute_epoch_at_slot::<P>(subscription.slot);

            self.subscriptions
                .entry(epoch)
                .or_default()
                .entry(subscription.validator_index)
                .or_default()
                .insert(subscription.committee_index, subscription);
        }
    }
}
