import {usingPlaygrounds} from './../util/playgrounds/index';
import {IKeyringPair} from '@polkadot/types/types';
import {normalizeEvents} from './util/helpers';
import {itEth, expect} from '../eth/util/playgrounds';

describe('evm collection sponsoring', () => {
  let donor: IKeyringPair;
  let alice: IKeyringPair;
  let nominal: bigint;

  before(async () => {
    await usingPlaygrounds(async (helper, privateKey) => {
      donor = privateKey('//Alice');
      nominal = helper.balance.getOneTokenNominal();
    });
  });

  beforeEach(async () => {
    await usingPlaygrounds(async (helper) => {
      [alice] = await helper.arrange.createAccounts([1000n], donor);
    });
  });

  itEth('sponsors mint transactions', async ({helper}) => {
    const collection = await helper.nft.mintCollection(alice, {tokenPrefix: 'spnr', permissions: {mintMode: true}});
    await collection.setSponsor(alice, alice.address);
    await collection.confirmSponsorship(alice);

    const minter = helper.eth.createAccount();
    expect(await helper.balance.getEthereum(minter)).to.equal(0n);

    const collectionAddress = helper.ethAddress.fromCollectionId(collection.collectionId);
    const contract = helper.ethNativeContract.collection(collectionAddress, 'nft', minter);

    await collection.addToAllowList(alice, {Ethereum: minter});

    const nextTokenId = await contract.methods.nextTokenId().call();
    expect(nextTokenId).to.equal('1');
    const result = await contract.methods.mint(minter, nextTokenId).send();
    const events = normalizeEvents(result.events);
    expect(events).to.be.deep.equal([
      {
        address: collectionAddress,
        event: 'Transfer',
        args: {
          from: '0x0000000000000000000000000000000000000000',
          to: minter,
          tokenId: nextTokenId,
        },
      },
    ]);
  });

  // TODO: Temprorary off. Need refactor
  // itWeb3('Set substrate sponsor', async ({api, web3, privateKeyWrapper}) => {
  //   const owner = await createEthAccountWithBalance(api, web3, privateKeyWrapper);
  //   const collectionHelpers = evmCollectionHelpers(web3, owner);
  //   let result = await collectionHelpers.methods.createNonfungibleCollection('Sponsor collection', '1', '1').send();
  //   const {collectionIdAddress, collectionId} = await getCollectionAddressFromResult(api, result);
  //   const sponsor = privateKeyWrapper('//Alice');
  //   const collectionEvm = evmCollection(web3, owner, collectionIdAddress);

  //   expect(await collectionEvm.methods.hasCollectionPendingSponsor().call({from: owner})).to.be.false;
  //   result = await collectionEvm.methods.setCollectionSponsorSubstrate(sponsor.addressRaw).send({from: owner});
  //   expect(await collectionEvm.methods.hasCollectionPendingSponsor().call({from: owner})).to.be.true;
    
  //   const confirmTx = await api.tx.unique.confirmSponsorship(collectionId);
  //   await submitTransactionAsync(sponsor, confirmTx);
  //   expect(await collectionEvm.methods.hasCollectionPendingSponsor().call({from: owner})).to.be.false;
    
  //   const sponsorTuple = await collectionEvm.methods.collectionSponsor().call({from: owner});
  //   expect(bigIntToSub(api, BigInt(sponsorTuple[1]))).to.be.eq(sponsor.address);
  // });

  itEth('Remove sponsor', async ({helper}) => {
    const owner = await helper.eth.createAccountWithBalance(donor);
    const collectionHelpers = helper.ethNativeContract.collectionHelpers(owner);

    let result = await collectionHelpers.methods.createNonfungibleCollection('Sponsor collection', '1', '1').send({value: Number(2n * nominal)});
    const collectionIdAddress = helper.ethAddress.normalizeAddress(result.events.CollectionCreated.returnValues.collectionId);
    const sponsor = await helper.eth.createAccountWithBalance(donor);
    const collectionEvm = helper.ethNativeContract.collection(collectionIdAddress, 'nft', owner);

    expect(await collectionEvm.methods.hasCollectionPendingSponsor().call({from: owner})).to.be.false;
    result = await collectionEvm.methods.setCollectionSponsor(sponsor).send({from: owner});
    expect(await collectionEvm.methods.hasCollectionPendingSponsor().call({from: owner})).to.be.true;
    
    await collectionEvm.methods.confirmCollectionSponsorship().send({from: sponsor});
    expect(await collectionEvm.methods.hasCollectionPendingSponsor().call({from: owner})).to.be.false;
    
    await collectionEvm.methods.removeCollectionSponsor().send({from: owner});
    
    const sponsorTuple = await collectionEvm.methods.collectionSponsor().call({from: owner});
    expect(sponsorTuple.field_0).to.be.eq('0x0000000000000000000000000000000000000000');
  });

  itEth('Sponsoring collection from evm address via access list', async ({helper, privateKey}) => {
    const owner = await helper.eth.createAccountWithBalance(donor);
    const collectionHelpers = helper.ethNativeContract.collectionHelpers(owner);

    let result = await collectionHelpers.methods.createNonfungibleCollection('Sponsor collection', '1', '1').send({value: Number(2n * nominal)});
    const collectionIdAddress = helper.ethAddress.normalizeAddress(result.events.CollectionCreated.returnValues.collectionId);
    const collectionId = helper.ethAddress.extractCollectionId(collectionIdAddress);
    const collection = helper.nft.getCollectionObject(collectionId);
    const sponsor = await helper.eth.createAccountWithBalance(donor);
    const collectionEvm = helper.ethNativeContract.collection(collectionIdAddress, 'nft', owner);

    result = await collectionEvm.methods.setCollectionSponsor(sponsor).send({from: owner});
    let collectionData = (await collection.getData())!;
    const ss58Format = helper.chain.getChainProperties().ss58Format;
    expect(collectionData.raw.sponsorship.Unconfirmed).to.be.eq(helper.address.ethToSubstrate(sponsor));
    await expect(collectionEvm.methods.confirmCollectionSponsorship().call()).to.be.rejectedWith('caller is not set as sponsor');

    await collectionEvm.methods.confirmCollectionSponsorship().send({from: sponsor});
    collectionData = (await collection.getData())!;
    expect(collectionData.raw.sponsorship.Confirmed).to.be.eq(helper.address.ethToSubstrate(sponsor));

    const user = helper.eth.createAccount();
    const nextTokenId = await collectionEvm.methods.nextTokenId().call();
    expect(nextTokenId).to.be.equal('1');

    const oldPermissions = (await collection.getData())!.raw.permissions; // (await getDetailedCollectionInfo(api, collectionId))!.permissions.toHuman();
    expect(oldPermissions.mintMode).to.be.false;
    expect(oldPermissions.access).to.be.equal('Normal');

    await collectionEvm.methods.setCollectionAccess(1 /*'AllowList'*/).send({from: owner});
    await collectionEvm.methods.addToCollectionAllowList(user).send({from: owner});
    await collectionEvm.methods.setCollectionMintMode(true).send({from: owner});

    const newPermissions = (await collection.getData())!.raw.permissions; // (await getDetailedCollectionInfo(api, collectionId))!.permissions.toHuman();
    expect(newPermissions.mintMode).to.be.true;
    expect(newPermissions.access).to.be.equal('AllowList');

    const ownerBalanceBefore = await helper.balance.getSubstrate(helper.address.ethToSubstrate(owner));
    const sponsorBalanceBefore = await helper.balance.getSubstrate(helper.address.ethToSubstrate(sponsor));

    {
      const nextTokenId = await collectionEvm.methods.nextTokenId().call();
      expect(nextTokenId).to.be.equal('1');
      const result = await collectionEvm.methods.mintWithTokenURI(
        user,
        nextTokenId,
        'Test URI',
      ).send({from: user});
      const events = normalizeEvents(result.events);

      expect(events).to.be.deep.equal([
        {
          address: collectionIdAddress,
          event: 'Transfer',
          args: {
            from: '0x0000000000000000000000000000000000000000',
            to: user,
            tokenId: nextTokenId,
          },
        },
      ]);

      const ownerBalanceAfter = await helper.balance.getSubstrate(await helper.address.ethToSubstrate(owner));
      const sponsorBalanceAfter = await helper.balance.getSubstrate(await helper.address.ethToSubstrate(sponsor));

      expect(await collectionEvm.methods.tokenURI(nextTokenId).call()).to.be.equal('Test URI');
      expect(ownerBalanceBefore).to.be.eq(ownerBalanceAfter);
      expect(sponsorBalanceBefore > sponsorBalanceAfter).to.be.true;
    }
  });

  // TODO: Temprorary off. Need refactor
  // itWeb3('Sponsoring collection from substrate address via access list', async ({api, web3, privateKeyWrapper}) => {
  //   const owner = await createEthAccountWithBalance(api, web3, privateKeyWrapper);
  //   const collectionHelpers = evmCollectionHelpers(web3, owner);
  //   const result = await collectionHelpers.methods.createNonfungibleCollection('Sponsor collection', '1', '1').send();
  //   const {collectionIdAddress, collectionId} = await getCollectionAddressFromResult(api, result);
  //   const sponsor = privateKeyWrapper('//Alice');
  //   const collectionEvm = evmCollection(web3, owner, collectionIdAddress);

  //   await collectionEvm.methods.setCollectionSponsorSubstrate(sponsor.addressRaw).send({from: owner});
    
  //   const confirmTx = await api.tx.unique.confirmSponsorship(collectionId);
  //   await submitTransactionAsync(sponsor, confirmTx);
    
  //   const user = createEthAccount(web3);
  //   const nextTokenId = await collectionEvm.methods.nextTokenId().call();
  //   expect(nextTokenId).to.be.equal('1');

  //   await collectionEvm.methods.setCollectionAccess(1 /*'AllowList'*/).send({from: owner});
  //   await collectionEvm.methods.addToCollectionAllowList(user).send({from: owner});
  //   await collectionEvm.methods.setCollectionMintMode(true).send({from: owner});

  //   const ownerBalanceBefore = await ethBalanceViaSub(api, owner);
  //   const sponsorBalanceBefore = (await getBalance(api, [sponsor.address]))[0];

  //   {
  //     const nextTokenId = await collectionEvm.methods.nextTokenId().call();
  //     expect(nextTokenId).to.be.equal('1');
  //     const result = await collectionEvm.methods.mintWithTokenURI(
  //       user,
  //       nextTokenId,
  //       'Test URI',
  //     ).send({from: user});
  //     const events = normalizeEvents(result.events);

  //     expect(events).to.be.deep.equal([
  //       {
  //         address: collectionIdAddress,
  //         event: 'Transfer',
  //         args: {
  //           from: '0x0000000000000000000000000000000000000000',
  //           to: user,
  //           tokenId: nextTokenId,
  //         },
  //       },
  //     ]);

  //     const ownerBalanceAfter = await ethBalanceViaSub(api, owner);
  //     const sponsorBalanceAfter = (await getBalance(api, [sponsor.address]))[0];

  //     expect(await collectionEvm.methods.tokenURI(nextTokenId).call()).to.be.equal('Test URI');
  //     expect(ownerBalanceBefore).to.be.eq(ownerBalanceAfter);
  //     expect(sponsorBalanceBefore > sponsorBalanceAfter).to.be.true;
  //   }
  // });

  itEth('Check that transaction via EVM spend money from sponsor address', async ({helper, privateKey}) => {
    const owner = await helper.eth.createAccountWithBalance(donor);
    const collectionHelpers = helper.ethNativeContract.collectionHelpers(owner);

    let result = await collectionHelpers.methods.createNonfungibleCollection('Sponsor collection', '1', '1').send({value: Number(2n * nominal)});
    const collectionIdAddress = helper.ethAddress.normalizeAddress(result.events.CollectionCreated.returnValues.collectionId);
    const collectionId = helper.ethAddress.extractCollectionId(collectionIdAddress);
    const collection = helper.nft.getCollectionObject(collectionId);
    const sponsor = await helper.eth.createAccountWithBalance(donor);
    const collectionEvm = helper.ethNativeContract.collection(collectionIdAddress, 'nft', owner);

    result = await collectionEvm.methods.setCollectionSponsor(sponsor).send();
    let collectionData = (await collection.getData())!;
    expect(collectionData.raw.sponsorship.Unconfirmed).to.be.eq(helper.address.ethToSubstrate(sponsor));
    await expect(collectionEvm.methods.confirmCollectionSponsorship().call()).to.be.rejectedWith('caller is not set as sponsor');

    const sponsorCollection = helper.ethNativeContract.collection(collectionIdAddress, 'nft', sponsor);
    await sponsorCollection.methods.confirmCollectionSponsorship().send();
    collectionData = (await collection.getData())!;
    expect(collectionData.raw.sponsorship.Confirmed).to.be.eq(helper.address.ethToSubstrate(sponsor));

    const user = helper.eth.createAccount();
    await collectionEvm.methods.addCollectionAdmin(user).send();
    
    const ownerBalanceBefore = await helper.balance.getSubstrate(helper.address.ethToSubstrate(owner));
    const sponsorBalanceBefore = await helper.balance.getSubstrate(helper.address.ethToSubstrate(sponsor));

    const userCollectionEvm = helper.ethNativeContract.collection(collectionIdAddress, 'nft', user);
    const nextTokenId = await userCollectionEvm.methods.nextTokenId().call();
    expect(nextTokenId).to.be.equal('1');
    result = await userCollectionEvm.methods.mintWithTokenURI(
      user,
      nextTokenId,
      'Test URI',
    ).send();

    const events = normalizeEvents(result.events);
    const address = helper.ethAddress.fromCollectionId(collectionId);

    expect(events).to.be.deep.equal([
      {
        address,
        event: 'Transfer',
        args: {
          from: '0x0000000000000000000000000000000000000000',
          to: user,
          tokenId: nextTokenId,
        },
      },
    ]);
    expect(await userCollectionEvm.methods.tokenURI(nextTokenId).call()).to.be.equal('Test URI');
  
    const ownerBalanceAfter = await helper.balance.getSubstrate(await helper.address.ethToSubstrate(owner));
    expect(ownerBalanceAfter).to.be.eq(ownerBalanceBefore);
    const sponsorBalanceAfter = await helper.balance.getSubstrate(await helper.address.ethToSubstrate(sponsor));
    expect(sponsorBalanceAfter < sponsorBalanceBefore).to.be.true;
  });
});
