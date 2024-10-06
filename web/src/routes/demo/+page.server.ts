import { fail } from '@sveltejs/kit';
import type { PageServerLoad} from './$types';
import type { GameBackendDesc } from '$lib/stores/replayStore';


export const load = (({ params }) => {
  return {
    demo : {
     players : [
      {
        bank : {
          sapphire: 1,
          emerald: 2,
          ruby: 3,
          onyx: 4,
          diamond: 5,
          gold: 6,
        },

        developments : {
          sapphire: 5,
          emerald: 4,
          ruby: 3,
          onyx: 2,
          diamond: 1,
        }, 

        totalPoints: 10,
        numReservedCards: 5,
      },
      {
        bank : {
          sapphire: 1,
          emerald: 2,
          ruby: 3,
          onyx: 4,
          diamond: 5,
          gold: 6,
        },

        developments : {
          sapphire: 5,
          emerald: 4,
          ruby: 3,
          onyx: 2,
          diamond: 1,
        }, 

        totalPoints: 10,
        numReservedCards: 5,
      },
      {
        bank : {
          sapphire: 1,
          emerald: 2,
          ruby: 3,
          onyx: 4,
          diamond: 5,
          gold: 6,
        },

        developments : {
          sapphire: 5,
          emerald: 4,
          ruby: 3,
          onyx: 2,
          diamond: 1,
        }, 

        totalPoints: 10,
        numReservedCards: 5,
      },
    ],

    board : { 
      deckCounts: [10,20,30],
      availableCards:[ 
      [
        
          {
           id: 0,
           cost : {
             sapphire: 1,
             emerald: 2,
             ruby: 3,
             onyx: 0,
             diamond: 0
           },
           points: 4,
           color: "sapphire",
          },
          {
           id: 0,
           cost : {
             sapphire: 1,
             emerald: 2,
             ruby: 3,
             onyx: 0,
             diamond: 0
           },
           points: 4,
           color: "sapphire",
          },
          {
           id: 0,
           cost : {
             sapphire: 1,
             emerald: 2,
             ruby: 3,
             onyx: 0,
             diamond: 0
           },
           points: 4,
           color: "sapphire",
          },
          {
           id: 0,
           cost : {
             sapphire: 1,
             emerald: 2,
             ruby: 3,
             onyx: 0,
             diamond: 0
           },
           points: 4,
           color: "sapphire",
          },
      ],
      [
        
          {
           id: 0,
           cost : {
             sapphire: 1,
             emerald: 2,
             ruby: 3,
             onyx: 0,
             diamond: 0
           },
           points: 4,
           color: "sapphire",
          },
          {
           id: 0,
           cost : {
             sapphire: 1,
             emerald: 2,
             ruby: 3,
             onyx: 0,
             diamond: 0
           },
           points: 4,
           color: "sapphire",
          },
          {
           id: 0,
           cost : {
             sapphire: 1,
             emerald: 2,
             ruby: 3,
             onyx: 0,
             diamond: 0
           },
           points: 4,
           color: "sapphire",
          },
          {
           id: 0,
           cost : {
             sapphire: 1,
             emerald: 2,
             ruby: 3,
             onyx: 0,
             diamond: 0
           },
           points: 4,
           color: "sapphire",
          },
      ],
      [
        
          {
           id: 0,
           cost : {
             sapphire: 1,
             emerald: 2,
             ruby: 3,
             onyx: 0,
             diamond: 0
           },
           points: 4,
           color: "sapphire",
          },
          {
           id: 0,
           cost : {
             sapphire: 1,
             emerald: 2,
             ruby: 3,
             onyx: 0,
             diamond: 0
           },
           points: 4,
           color: "sapphire",
          },
          {
           id: 0,
           cost : {
             sapphire: 1,
             emerald: 2,
             ruby: 3,
             onyx: 0,
             diamond: 0
           },
           points: 4,
           color: "sapphire",
          },
          {
           id: 0,
           cost : {
             sapphire: 1,
             emerald: 2,
             ruby: 3,
             onyx: 0,
             diamond: 0
           },
           points: 4,
           color: "sapphire",
          },
      ],
      ],
      
      nobles : [ 
       { 
        cost: {
          sapphire: 3,
          emerald: 3,
          ruby: 3,
          onyx: 0,
          diamond: 0,
        },
       },
       {
        cost: {
          sapphire: 3,
          emerald: 3,
          ruby: 3,
          onyx: 0,
          diamond: 0,
        },
       },
       {
        cost: {
          sapphire: 3,
          emerald: 3,
          ruby: 3,
          onyx: 0,
          diamond: 0,
        },
       },
       {
        cost: {
          sapphire: 3,
          emerald: 3,
          ruby: 3,
          onyx: 0,
          diamond: 0,
        },
       },
      ],

      bank : {
          sapphire: 1,
          emerald: 2,
          ruby: 3,
          onyx: 4,
          diamond: 5,
          gold: 6,
      },

      },
      turnNumber: 0,
      currentPlayer : 0
    } 
  }
}) satisfies PageServerLoad;

