import { writable } from 'svelte/store';

export type Gem = "sapphire" | "emerald" | "ruby" | "onyx" | "diamond" | "gold";
export type Cost = "sapphire" | "emerald" | "ruby" | "onyx" | "diamond" ;

export type Gems = {
  sapphire: number,
  emerald: number,
  ruby: number,
  onyx: number,
  diamond: number,
  gold: number,
}

export type Costs = {
  sapphire: number,
  emerald: number,
  ruby: number,
  onyx: number,
  diamond: number,
}


//TODO: unify the types of the backend and frontend

export type CardDesc = {
  points: number,
  gem: Cost,
  cost: Map<Cost, number>,
}

export type PlayerNewDesc = {
  bank : Gems,
  developments : Costs,
  totalPoints : number,
  numReservedCards : number,
}

export type NobleNewDesc = {
  cost : Costs,
}

export type CardNewDesc = {
  id: number,
  cost: Costs,
  points: number,
  color: Gem
}

export type BoardNewDesc = {
  deckCounts: Array<number>,
  availableCards: Array<CardNewDesc>,
  nobles: Array<NobleNewDesc>,
  bank: Gems,
  currentPlayer: number
}

export type GameBackendDesc = {
  board: BoardNewDesc, 
  players: Array<PlayerNewDesc>,
  turnNumber : number,
  currentPlayer: number,
}

export type PlayerDesc = {
  name: string,
  developments: Map<Gem, number>,
  gems: Map<Gem, number>,
  numReservedCards: number
  totalPoints: number,
  currentPlayer: boolean,
}

export type NobleReq = {
  gemName: Gem,
  gemCount: number,
}

export type NobleDesc = {
  requirements: Array<NobleReqs>,
}

export type BankDesc = {
  gemName: Gem,
  gemCount: number,
}

export let turnNumber = writable(1);
export let nobles = writable(Array<NobleDesc>());
export let bank = writable(Array<BankDesc>());
export let number_players = writable(4);
export let players = writable(Array<PlayerDesc>());
export let cards = writable(Array<Array<CardDesc>>());
export let deckCounts = writable([0, 0, 0]);

// TODO: update this to use enums or string so we don't have to look at this reference again
// TODO: merge into one big API call rather than multiple to reduce latency 
// Match the conventions of the frontend gems
//
//          color    : index
//	 white (diamond) : 0
//	 blue (sapphire) : 1
//	 green (emerald) : 2
//	 red (ruby)      : 3
//	 black (onyx)    : 4
//	 yellow (gold)   : 5
export function indexToGem(index) : Gem | undefined {
    switch (index) {
        case 0:
            return 'diamond';
        case 1:
            return 'sapphire';
        case 2:
            return 'emerald';
        case 3:
            return 'ruby';
        case 4:
            return 'onyx';
        case 5:
            return 'gold';
    }
}



export function updateGameBanks(update : GameBackendDesc | undefined, turn : number) {
  
  if (update == undefined) { return; }
  let bankDescriptions : Array<BankDesc> = Array();
  for (let [gemName, count] of gemsToMap(update.board.bank).entries()) {
    let bankDesc : BankDesc = {
      gemName : gemName,
      gemCount : count,
    }
    bankDescriptions.push(bankDesc);
  }

   bank.update(() => bankDescriptions);
}

export function updateGameNobles(update: GameBackendDesc | undefined, turn : number) {

  if (update == undefined) { return; }


  let new_nobles : Array<NobleDesc> = Array();
  update.board.nobles.forEach( (nobleReqs : Array<NobleNewDesc>) => {
    let noble : NobleDesc = {
      requirements : Array(),
    }

    let  nobleMap = costsToMap(nobleReqs.cost);
    for (let [gemName, gemCount] of nobleMap.entries()) {

      let nobleReq : NobleReq = {
        gemName : gemName,
        gemCount : gemCount,
      }
      if (gemCount > 0) {
        noble.requirements.push(nobleReq);
      }
    }
    new_nobles.push(noble);

  });

  nobles.update(() => new_nobles);

}

export function updateGamePlayers(update: GameBackendDesc | undefined, turn : number) {

  if (update == undefined) { return; }
  let newPlayers : Array<PlayerDesc> = Array();

  update.players.forEach((player : PlayerNewDesc, id :number) => { 

      let developments = costsToMap(player.developments);
      let gems = gemsToMap(player.bank);
      let totalPoints = player.totalPoints;
      let numReservedCards = player.numReservedCards;

      let playerDesc = {name : "Player " + id, 
                        developments : developments, 
                        gems : gems, 
                        totalPoints : totalPoints, 
                        numReservedCards : numReservedCards,
                        currentPlayer : id == update.currentPlayer
      };
      newPlayers.push(playerDesc);

    });

    players.update(() => newPlayers);
}

export function costsToMap(costs : Costs) : Map<Cost, number> {
  let map = new Map<Cost, number>();
  map.set('sapphire', costs.sapphire);
  map.set('emerald', costs.emerald);
  map.set('ruby', costs.ruby);
  map.set('onyx', costs.onyx);
  map.set('diamond', costs.diamond);
  return map;
}

export function gemsToMap(gems : Gems) : Map<Gem, number> {
  let map = new Map<Gem, number>();
  map.set('sapphire', gems.sapphire);
  map.set('emerald', gems.emerald);
  map.set('ruby', gems.ruby);
  map.set('onyx', gems.onyx);
  map.set('diamond', gems.diamond);
  map.set('gold', gems.gold);
  return map;
}

export function updateGameCards(update : GameBackendDesc | undefined, turn : number) {
        if (update == undefined) { return; }
        let new_cards : Array<Array<CardDesc>> = [];


        update.board.availableCards.forEach((row : Array<CardNewDesc>) => {
          let cardRow : Array<CardDesc> = [];
          row.forEach((card, _) => {

            let cost = costsToMap(card.cost);

            cost.forEach((value, key) => {
              if (value == 0) {
                cost.delete(key);
              }
            });

            let cardDesc : CardDesc = {
              points: card.points,
              gem: card.color,
              cost: cost
            }
            cardRow.push(cardDesc);
          });
          new_cards.push(cardRow);

        });
       cards.update(() => new_cards); 
  }

export function updateGameDeckCounts(update : GameBackendDesc | undefined, turn : number) {
      if (update == undefined) { return; }
      let new_decks = [0, 0, 0];
      
      update.board.deckCounts.forEach((deck : number, index) => {
          // Map tiers 0 to 2 to indices 2 to 0 for the frontend view
          // TODO: this isn't that nice
          index = 2 - index;
          new_decks[index] = deck;
      });

      deckCounts.update(() => new_decks);
}

