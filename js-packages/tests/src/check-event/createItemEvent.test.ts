// Copyright 2019-2022 Unique Network (Gibraltar) Ltd.
// This file is part of Unique Network.

// Unique Network is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Unique Network is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Unique Network. If not, see <http://www.gnu.org/licenses/>.

// https://unique-network.readthedocs.io/en/latest/jsapi.html#setchainlimits
import type {IKeyringPair} from '@polkadot/types/types';
import {itSub, usingPlaygrounds, expect} from '../util/index.js';
import type {IEvent} from '@unique/playgrounds/src/types.js';

describe('Create Item event ', () => {
  let alice: IKeyringPair;
  before(async () => {
    await usingPlaygrounds(async (helper, privateKey) => {
      const donor = await privateKey({url: import.meta.url});
      [alice] = await helper.arrange.createAccounts([10n], donor);
    });
  });
  itSub('Check event from createItem(): ', async ({helper}) => {
    const collection = await helper.nft.mintCollection(alice, {name: 'test', description: 'test', tokenPrefix: 'test'});
    await collection.mintToken(alice, {Substrate: alice.address});
    await helper.wait.newBlocks(1);
    const event = helper.chainLog[helper.chainLog.length - 1].events as IEvent[];
    const eventStrings = event.map(e => `${e.section}.${e.method}`);

    expect(eventStrings).to.contains('common.ItemCreated');
    expect(eventStrings).to.contains('treasury.Deposit');
    expect(eventStrings).to.contains('system.ExtrinsicSuccess');
  });
});
