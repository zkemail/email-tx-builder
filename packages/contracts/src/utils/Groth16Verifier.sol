// SPDX-License-Identifier: GPL-3.0
/*
    Copyright 2021 0KIMS association.

    This file is generated with [snarkJS](https://github.com/iden3/snarkjs).

    snarkJS is a free software: you can redistribute it and/or modify it
    under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    snarkJS is distributed in the hope that it will be useful, but WITHOUT
    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
    or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
    License for more details.

    You should have received a copy of the GNU General Public License
    along with snarkJS. If not, see <https://www.gnu.org/licenses/>.
*/

pragma solidity >=0.7.0 <0.9.0;

contract Groth16Verifier {
    // Scalar field size
    uint256 constant r    = 21888242871839275222246405745257275088548364400416034343698204186575808495617;
    // Base field size
    uint256 constant q   = 21888242871839275222246405745257275088696311157297823662689037894645226208583;

    // Verification Key data
    uint256 constant alphax  = 20491192805390485299153009773594534940189261866228447918068658471970481763042;
    uint256 constant alphay  = 9383485363053290200918347156157836566562967994039712273449902621266178545958;
    uint256 constant betax1  = 4252822878758300859123897981450591353533073413197771768651442665752259397132;
    uint256 constant betax2  = 6375614351688725206403948262868962793625744043794305715222011528459656738731;
    uint256 constant betay1  = 21847035105528745403288232691147584728191162732299865338377159692350059136679;
    uint256 constant betay2  = 10505242626370262277552901082094356697409835680220590971873171140371331206856;
    uint256 constant gammax1 = 11559732032986387107991004021392285783925812861821192530917403151452391805634;
    uint256 constant gammax2 = 10857046999023057135944570762232829481370756359578518086990519993285655852781;
    uint256 constant gammay1 = 4082367875863433681332203403145435568316851327593401208105741076214120093531;
    uint256 constant gammay2 = 8495653923123431417604973247489272438418190587263600148770280649306958101930;
    uint256 constant deltax1 = 15782391977335357081340382271950726439491538703900326098255114680175072199333;
    uint256 constant deltax2 = 17317246753771344552223817973688041228547872025713834810859369955856471183807;
    uint256 constant deltay1 = 8391802864724428140107526016382684103598390293137169159963631808215713227972;
    uint256 constant deltay2 = 20785143699213063466112127963830605308646889510634150156361272072259651210564;

    
    uint256 constant IC0x = 4758508624079147551848696777612109704815150734593065674236173814775997488279;
    uint256 constant IC0y = 20292299590563400527849202850411820779414915472454788819358002091889424462932;
    
    uint256 constant IC1x = 5301255377873624609658894467438982493085703692646930535561935781883226209492;
    uint256 constant IC1y = 6731142610883878715377462519935548157423353736582256473979463807448069660562;
    
    uint256 constant IC2x = 1327835035772326734093504732823788908156198591294121598267556153703708112951;
    uint256 constant IC2y = 16686564593924341069928355568194146113760126601557761707915083005542806137361;
    
    uint256 constant IC3x = 6510529089759675619659303808746989332328265014498267965473701990230310027177;
    uint256 constant IC3y = 19800776575653500360323335352355897137807622935165771474939552514024343648863;
    
    uint256 constant IC4x = 21118744323842469653415987285012129228709442831132450653205602258144817527063;
    uint256 constant IC4y = 18695165737943748773492309968416077403911122483560683563763064683839247377256;
    
    uint256 constant IC5x = 15592752787325189334205034055928402697589287290075190834208001577263120002690;
    uint256 constant IC5y = 18628612253965241261386051179384621921694001120191482885423401796259127817801;
    
    uint256 constant IC6x = 16747412216920433963180372951394492285791858551143210584707337473557638643863;
    uint256 constant IC6y = 15772996125829825910582707271113479663936632141420719711558988843884988632711;
    
    uint256 constant IC7x = 12831190282763434566110380400048223334592029617858121145087101520789866729132;
    uint256 constant IC7y = 19917300570048601437948738626243787397157818086550177680022468877684201135703;
    
    uint256 constant IC8x = 11878438249250569038751525547052431500596908394348469324260852499432188564731;
    uint256 constant IC8y = 19946202317480644056921749477694930608615216240065673145745570305474669038328;
    
    uint256 constant IC9x = 10459691155485788197993648398511773684990482262539700115477552635768651243299;
    uint256 constant IC9y = 13423032957648635928258827828389026990781489495864803609851670804958705866105;
    
    uint256 constant IC10x = 18101745929249219297071817655002198120068044107357823169035464898813884480930;
    uint256 constant IC10y = 21631408229198493952336504786409122883815055472752397690167736727824017260089;
    
    uint256 constant IC11x = 83820076544104898283766797612130382283699538524518502894236657242377534202;
    uint256 constant IC11y = 11170308815906799885840923551925730620222973921776498937842003543183592075661;
    
    uint256 constant IC12x = 6071270783796128901939061028854161790673958186675052248417577837089569121329;
    uint256 constant IC12y = 6984200819922380808553234519991562613246466151705409450937332410182058366892;
    
    uint256 constant IC13x = 732717785825945150782947762308498168841296284500919687992269578318653637470;
    uint256 constant IC13y = 14175860620963381986105149001664253152839970478566225473262731158872223944582;
    
    uint256 constant IC14x = 14614996547359724787858153367018342810208505887149210446232105432764997433302;
    uint256 constant IC14y = 3776948758208685861840266116249062583175248874271030017627409718431492615001;
    
    uint256 constant IC15x = 2017132840109614298935123512194590970712331953522882079385764196856919087507;
    uint256 constant IC15y = 20272817959207125486537094018895811803597356660073062742417162682235764786299;
    
    uint256 constant IC16x = 12471931745840573922066675686519541049354651078411919509073876762441914712289;
    uint256 constant IC16y = 20475038549658621859864674958920648095571925682102205870226707059284014014381;
    
    uint256 constant IC17x = 20981229637943417068258989039428801493420519102055320405534922464508182161034;
    uint256 constant IC17y = 15418917071895500497312264680330587838861217092134535919797411340196259455025;
    
    uint256 constant IC18x = 6944385259292800103105962196504075625003113657352113624858361537841910782011;
    uint256 constant IC18y = 1702320754116932445747112548948789744661521152544633520021651714428407913311;
    
    uint256 constant IC19x = 737442371830422947732113368157410357257659441996773615309882855812810443818;
    uint256 constant IC19y = 18871169426510379702260011535993449415787251365125580566174191880997193075550;
    
    uint256 constant IC20x = 19252433510522456048429570422801818582250986652581849977020659334976857461176;
    uint256 constant IC20y = 20656623484059380818731527144387003756806901714236682988252571010949718527782;
    
    uint256 constant IC21x = 11894224518203601173945622281352527508930029935795176687578361209866191695638;
    uint256 constant IC21y = 13303129793618588894119723163981925084339646127786019902008827830941610400686;
    
    uint256 constant IC22x = 1311957606959953407437262424843052234425858596996793233096022467855962941996;
    uint256 constant IC22y = 8065257524499341410956553099517999014244421650968997036958393120456018562580;
    
    uint256 constant IC23x = 14803102557327319143871415193302695716074406086585367383601810762465904066470;
    uint256 constant IC23y = 17818638482974298552031364086378913676395115067922992597700112226529522132265;
    
    uint256 constant IC24x = 13973286712675820978976695545697023120568071203431684282675587248050359446678;
    uint256 constant IC24y = 16321877229628853324068631491700697370319793825872061604570284841271665494439;
    
    uint256 constant IC25x = 21868419289951258404784760523728531072066951489319560957785447430486538061370;
    uint256 constant IC25y = 17365206831581292204418387715844742894501643941979356724969809370270566600909;
    
    uint256 constant IC26x = 20941022606891065068758914446259133027953311918644235373691066201030181585375;
    uint256 constant IC26y = 12190179019490652481089502336147007630342430283620057040512130079973697703186;
    
    uint256 constant IC27x = 8545783269396868086251000709690501183625871807947171240958874121297693202386;
    uint256 constant IC27y = 14388103178148262983884686855764509952092040853878285067522959895943852293675;
    
    uint256 constant IC28x = 1249519061804311670473043337588322421500991571401461174981469339039753691305;
    uint256 constant IC28y = 2804341221933993149877828236240252550207179554316520108917229655711269401179;
    
    uint256 constant IC29x = 20116849424566986506128411084923886300121092393728226283817410910744656509983;
    uint256 constant IC29y = 16567882698783363669659058390691368563443359407565246637420846913973746054327;
    
    uint256 constant IC30x = 15557694771863488068649332511593290955330774836995608004536462735486870729131;
    uint256 constant IC30y = 17945504381872363506688052789805874163286509742917680206032854082683285819365;
    
    uint256 constant IC31x = 15264678984994483564475560102604402606260728386345108906773354414523742172822;
    uint256 constant IC31y = 15834630292784289674251776956533470723805124163781492376299163220974715478073;
    
    uint256 constant IC32x = 6855109818115692526577716268048225989267866981013806997939847903383709464412;
    uint256 constant IC32y = 2575023964968419065285843838845754078868133882183801076975234826549126094381;
    
    uint256 constant IC33x = 16954236169562573998872957799972742735715577948015276093283764267467221567110;
    uint256 constant IC33y = 17602789747298367755981601739171202984883208093574349815719178503143859798820;
    
    uint256 constant IC34x = 15422954576418261534947981972896945018747789000238233860636954680201949238850;
    uint256 constant IC34y = 18147784190828652842359193054831937523841097038833106432281414972197690312053;
    
 
    // Memory data
    uint16 constant pVk = 0;
    uint16 constant pPairing = 128;

    uint16 constant pLastMem = 896;

    function verifyProof(uint[2] calldata _pA, uint[2][2] calldata _pB, uint[2] calldata _pC, uint[34] calldata _pubSignals) public view returns (bool) {
        assembly {
            function checkField(v) {
                if iszero(lt(v, r)) {
                    mstore(0, 0)
                    return(0, 0x20)
                }
            }
            
            // G1 function to multiply a G1 value(x,y) to value in an address
            function g1_mulAccC(pR, x, y, s) {
                let success
                let mIn := mload(0x40)
                mstore(mIn, x)
                mstore(add(mIn, 32), y)
                mstore(add(mIn, 64), s)

                success := staticcall(sub(gas(), 2000), 7, mIn, 96, mIn, 64)

                if iszero(success) {
                    mstore(0, 0)
                    return(0, 0x20)
                }

                mstore(add(mIn, 64), mload(pR))
                mstore(add(mIn, 96), mload(add(pR, 32)))

                success := staticcall(sub(gas(), 2000), 6, mIn, 128, pR, 64)

                if iszero(success) {
                    mstore(0, 0)
                    return(0, 0x20)
                }
            }

            function checkPairing(pA, pB, pC, pubSignals, pMem) -> isOk {
                let _pPairing := add(pMem, pPairing)
                let _pVk := add(pMem, pVk)

                mstore(_pVk, IC0x)
                mstore(add(_pVk, 32), IC0y)

                // Compute the linear combination vk_x
                
                g1_mulAccC(_pVk, IC1x, IC1y, calldataload(add(pubSignals, 0)))
                
                g1_mulAccC(_pVk, IC2x, IC2y, calldataload(add(pubSignals, 32)))
                
                g1_mulAccC(_pVk, IC3x, IC3y, calldataload(add(pubSignals, 64)))
                
                g1_mulAccC(_pVk, IC4x, IC4y, calldataload(add(pubSignals, 96)))
                
                g1_mulAccC(_pVk, IC5x, IC5y, calldataload(add(pubSignals, 128)))
                
                g1_mulAccC(_pVk, IC6x, IC6y, calldataload(add(pubSignals, 160)))
                
                g1_mulAccC(_pVk, IC7x, IC7y, calldataload(add(pubSignals, 192)))
                
                g1_mulAccC(_pVk, IC8x, IC8y, calldataload(add(pubSignals, 224)))
                
                g1_mulAccC(_pVk, IC9x, IC9y, calldataload(add(pubSignals, 256)))
                
                g1_mulAccC(_pVk, IC10x, IC10y, calldataload(add(pubSignals, 288)))
                
                g1_mulAccC(_pVk, IC11x, IC11y, calldataload(add(pubSignals, 320)))
                
                g1_mulAccC(_pVk, IC12x, IC12y, calldataload(add(pubSignals, 352)))
                
                g1_mulAccC(_pVk, IC13x, IC13y, calldataload(add(pubSignals, 384)))
                
                g1_mulAccC(_pVk, IC14x, IC14y, calldataload(add(pubSignals, 416)))
                
                g1_mulAccC(_pVk, IC15x, IC15y, calldataload(add(pubSignals, 448)))
                
                g1_mulAccC(_pVk, IC16x, IC16y, calldataload(add(pubSignals, 480)))
                
                g1_mulAccC(_pVk, IC17x, IC17y, calldataload(add(pubSignals, 512)))
                
                g1_mulAccC(_pVk, IC18x, IC18y, calldataload(add(pubSignals, 544)))
                
                g1_mulAccC(_pVk, IC19x, IC19y, calldataload(add(pubSignals, 576)))
                
                g1_mulAccC(_pVk, IC20x, IC20y, calldataload(add(pubSignals, 608)))
                
                g1_mulAccC(_pVk, IC21x, IC21y, calldataload(add(pubSignals, 640)))
                
                g1_mulAccC(_pVk, IC22x, IC22y, calldataload(add(pubSignals, 672)))
                
                g1_mulAccC(_pVk, IC23x, IC23y, calldataload(add(pubSignals, 704)))
                
                g1_mulAccC(_pVk, IC24x, IC24y, calldataload(add(pubSignals, 736)))
                
                g1_mulAccC(_pVk, IC25x, IC25y, calldataload(add(pubSignals, 768)))
                
                g1_mulAccC(_pVk, IC26x, IC26y, calldataload(add(pubSignals, 800)))
                
                g1_mulAccC(_pVk, IC27x, IC27y, calldataload(add(pubSignals, 832)))
                
                g1_mulAccC(_pVk, IC28x, IC28y, calldataload(add(pubSignals, 864)))
                
                g1_mulAccC(_pVk, IC29x, IC29y, calldataload(add(pubSignals, 896)))
                
                g1_mulAccC(_pVk, IC30x, IC30y, calldataload(add(pubSignals, 928)))
                
                g1_mulAccC(_pVk, IC31x, IC31y, calldataload(add(pubSignals, 960)))
                
                g1_mulAccC(_pVk, IC32x, IC32y, calldataload(add(pubSignals, 992)))
                
                g1_mulAccC(_pVk, IC33x, IC33y, calldataload(add(pubSignals, 1024)))
                
                g1_mulAccC(_pVk, IC34x, IC34y, calldataload(add(pubSignals, 1056)))
                

                // -A
                mstore(_pPairing, calldataload(pA))
                mstore(add(_pPairing, 32), mod(sub(q, calldataload(add(pA, 32))), q))

                // B
                mstore(add(_pPairing, 64), calldataload(pB))
                mstore(add(_pPairing, 96), calldataload(add(pB, 32)))
                mstore(add(_pPairing, 128), calldataload(add(pB, 64)))
                mstore(add(_pPairing, 160), calldataload(add(pB, 96)))

                // alpha1
                mstore(add(_pPairing, 192), alphax)
                mstore(add(_pPairing, 224), alphay)

                // beta2
                mstore(add(_pPairing, 256), betax1)
                mstore(add(_pPairing, 288), betax2)
                mstore(add(_pPairing, 320), betay1)
                mstore(add(_pPairing, 352), betay2)

                // vk_x
                mstore(add(_pPairing, 384), mload(add(pMem, pVk)))
                mstore(add(_pPairing, 416), mload(add(pMem, add(pVk, 32))))


                // gamma2
                mstore(add(_pPairing, 448), gammax1)
                mstore(add(_pPairing, 480), gammax2)
                mstore(add(_pPairing, 512), gammay1)
                mstore(add(_pPairing, 544), gammay2)

                // C
                mstore(add(_pPairing, 576), calldataload(pC))
                mstore(add(_pPairing, 608), calldataload(add(pC, 32)))

                // delta2
                mstore(add(_pPairing, 640), deltax1)
                mstore(add(_pPairing, 672), deltax2)
                mstore(add(_pPairing, 704), deltay1)
                mstore(add(_pPairing, 736), deltay2)


                let success := staticcall(sub(gas(), 2000), 8, _pPairing, 768, _pPairing, 0x20)

                isOk := and(success, mload(_pPairing))
            }

            let pMem := mload(0x40)
            mstore(0x40, add(pMem, pLastMem))

            // Validate that all evaluations ∈ F
            
            checkField(calldataload(add(_pubSignals, 0)))
            
            checkField(calldataload(add(_pubSignals, 32)))
            
            checkField(calldataload(add(_pubSignals, 64)))
            
            checkField(calldataload(add(_pubSignals, 96)))
            
            checkField(calldataload(add(_pubSignals, 128)))
            
            checkField(calldataload(add(_pubSignals, 160)))
            
            checkField(calldataload(add(_pubSignals, 192)))
            
            checkField(calldataload(add(_pubSignals, 224)))
            
            checkField(calldataload(add(_pubSignals, 256)))
            
            checkField(calldataload(add(_pubSignals, 288)))
            
            checkField(calldataload(add(_pubSignals, 320)))
            
            checkField(calldataload(add(_pubSignals, 352)))
            
            checkField(calldataload(add(_pubSignals, 384)))
            
            checkField(calldataload(add(_pubSignals, 416)))
            
            checkField(calldataload(add(_pubSignals, 448)))
            
            checkField(calldataload(add(_pubSignals, 480)))
            
            checkField(calldataload(add(_pubSignals, 512)))
            
            checkField(calldataload(add(_pubSignals, 544)))
            
            checkField(calldataload(add(_pubSignals, 576)))
            
            checkField(calldataload(add(_pubSignals, 608)))
            
            checkField(calldataload(add(_pubSignals, 640)))
            
            checkField(calldataload(add(_pubSignals, 672)))
            
            checkField(calldataload(add(_pubSignals, 704)))
            
            checkField(calldataload(add(_pubSignals, 736)))
            
            checkField(calldataload(add(_pubSignals, 768)))
            
            checkField(calldataload(add(_pubSignals, 800)))
            
            checkField(calldataload(add(_pubSignals, 832)))
            
            checkField(calldataload(add(_pubSignals, 864)))
            
            checkField(calldataload(add(_pubSignals, 896)))
            
            checkField(calldataload(add(_pubSignals, 928)))
            
            checkField(calldataload(add(_pubSignals, 960)))
            
            checkField(calldataload(add(_pubSignals, 992)))
            
            checkField(calldataload(add(_pubSignals, 1024)))
            
            checkField(calldataload(add(_pubSignals, 1056)))
            

            // Validate all evaluations
            let isValid := checkPairing(_pA, _pB, _pC, _pubSignals, pMem)

            mstore(0, isValid)
             return(0, 0x20)
         }
     }
 }
