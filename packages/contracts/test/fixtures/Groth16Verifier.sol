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
    uint256 constant r =
        21888242871839275222246405745257275088548364400416034343698204186575808495617;
    // Base field size
    uint256 constant q =
        21888242871839275222246405745257275088696311157297823662689037894645226208583;

    // Verification Key data
    uint256 constant alphax =
        16428432848801857252194528405604668803277877773566238944394625302971855135431;
    uint256 constant alphay =
        16846502678714586896801519656441059708016666274385668027902869494772365009666;
    uint256 constant betax1 =
        3182164110458002340215786955198810119980427837186618912744689678939861918171;
    uint256 constant betax2 =
        16348171800823588416173124589066524623406261996681292662100840445103873053252;
    uint256 constant betay1 =
        4920802715848186258981584729175884379674325733638798907835771393452862684714;
    uint256 constant betay2 =
        19687132236965066906216944365591810874384658708175106803089633851114028275753;
    uint256 constant gammax1 =
        11559732032986387107991004021392285783925812861821192530917403151452391805634;
    uint256 constant gammax2 =
        10857046999023057135944570762232829481370756359578518086990519993285655852781;
    uint256 constant gammay1 =
        4082367875863433681332203403145435568316851327593401208105741076214120093531;
    uint256 constant gammay2 =
        8495653923123431417604973247489272438418190587263600148770280649306958101930;
    uint256 constant deltax1 =
        10264548714561178023199180875646797929252572511673269331763091491411145960772;
    uint256 constant deltax2 =
        18311575787560507146958693096350917774373079228269069858880619696687209239156;
    uint256 constant deltay1 =
        15364192743383586712577160784335601317663308697309919234290044903285384079519;
    uint256 constant deltay2 =
        865431301441011786816114454471335882030231200282491476064342442243395470609;

    uint256 constant IC0x =
        15202293611052609963793495512467254159525935701952156579860017919463680904977;
    uint256 constant IC0y =
        21661005325338063078677013303929203363922048001197630562247322848427814167028;

    uint256 constant IC1x =
        6452955991179147115246107219429817574024379162049674324925726287356258935551;
    uint256 constant IC1y =
        1046765160185674313241656418982162550792459336028800222786437867754715411953;

    uint256 constant IC2x =
        19561639896993505362129801692608602738073625583833671944648945381547014922029;
    uint256 constant IC2y =
        16578191314849960975440386484587520749095058197266524744875542035396142224979;

    uint256 constant IC3x =
        6848044820898772807251245352999299723732963485322714031830239563848700077224;
    uint256 constant IC3y =
        2651857088234354311303576559129256483017639270071990898705867309911277075744;

    uint256 constant IC4x =
        2368020592197748386535215526517962820178664923849697226874744178021374761681;
    uint256 constant IC4y =
        12405348435978174377662066841808892817975298275719433954945459872424998071839;

    uint256 constant IC5x =
        12538400703934928967761620107286471911808571054792037306376531644420655213601;
    uint256 constant IC5y =
        5250913866312777687398128331360175028718921856999591936365321282877989184554;

    uint256 constant IC6x =
        21618258444494928915811318893263107924740504125944838336461987588498540593641;
    uint256 constant IC6y =
        2413936983943311614040050831359105896959820096643403180384172770646720886542;

    uint256 constant IC7x =
        9712390331435193528924418117411029699629191054855717365288341052594339984129;
    uint256 constant IC7y =
        2901890001751330239457872918991982397824036405720917553836999774204044455623;

    uint256 constant IC8x =
        15974463782248120842322037142570537184584834522125034585921335575378575808731;
    uint256 constant IC8y =
        907969419061262461168959968625500836314075317901922012544307573862997420676;

    uint256 constant IC9x =
        593520515785869763751321392905809175096343673588301633790700045661419391225;
    uint256 constant IC9y =
        13590538885742242839583031853957508852299822005933772806256906067517430335152;

    uint256 constant IC10x =
        5126169742944031682914481076896674210863618620274411658445473618966611691186;
    uint256 constant IC10y =
        11064730524220376902728653747571499563397123195029671857363423441040030329460;

    uint256 constant IC11x =
        7259905309705066795127861568175002956910167005984267721335918540367744711170;
    uint256 constant IC11y =
        18060290263760873381165903451091718347725372544343300414984343301594481434248;

    uint256 constant IC12x =
        20630593989712907614329227879257173969981449059516858515932180120315541561010;
    uint256 constant IC12y =
        21233726060276660013525937027407792589490085528161401072805191653312390912746;

    uint256 constant IC13x =
        21561014780309254670962294569241599431921262891245404626628451318723403477889;
    uint256 constant IC13y =
        14995299258320870787097856185937491178710155807159202012928750913696984420098;

    uint256 constant IC14x =
        15280115641601385582695107490819213341046876675730545125336451012678022158547;
    uint256 constant IC14y =
        3320140604782147564819602725346016709067873440446178893087399954861063843693;

    uint256 constant IC15x =
        17581987119415701930284019851842751354356227331927930316001806538430223682555;
    uint256 constant IC15y =
        11705390309586641220857009322374633789357714112517459725585094189993550500502;

    uint256 constant IC16x =
        8730931148876341537574019799386064417181454807640745687902577256185621504752;
    uint256 constant IC16y =
        17468060086293611644001868640796316902388876060470858956528235204288140360575;

    uint256 constant IC17x =
        8861089091657651141391517128205040995769246836643614743523809351020752533882;
    uint256 constant IC17y =
        2480113299000827096835342659230864215648139204515758667998468461670896051319;

    uint256 constant IC18x =
        17939336806984939360292212764988364743099223651317677582855107905643819103567;
    uint256 constant IC18y =
        11345872484067084307070331806785082787094115795567848618971963992433132980105;

    uint256 constant IC19x =
        17872444896804360649027636059356355146821158031152682850483070881386442953397;
    uint256 constant IC19y =
        5216957782766764544695058326427041135266417604395823288983956514263254735763;

    uint256 constant IC20x =
        18614603351333566941561341323477869570590042690081796211106802966274611143634;
    uint256 constant IC20y =
        6117879660203592329824485391814611137557595216579378075477245855577866972763;

    uint256 constant IC21x =
        21778815968256885187175750646471987903451308718075604716026343917461433675766;
    uint256 constant IC21y =
        15945045594185327054143762990907113241468460444847255176566320744907818510192;

    uint256 constant IC22x =
        9420414154372262062101448921210266527775069404690917420602160684490398686167;
    uint256 constant IC22y =
        17925640305786657666580892876791489338459889323848286283099304226197949920700;

    uint256 constant IC23x =
        1970536739758456323127804199327246259716526231320855273266665875199442007659;
    uint256 constant IC23y =
        16758858790649768578768857243572869296075602760261863455857563135567659592212;

    uint256 constant IC24x =
        10923895606766058814188172080806452656128446108501130817389994541753802960573;
    uint256 constant IC24y =
        20801878558750282858098578295551361461395721372368916692122569109866370311341;

    uint256 constant IC25x =
        8489780023908430543526842780524979232609226423787632005008601601225833157113;
    uint256 constant IC25y =
        4882582226086767973432420175935156928604793616650145375822401109295644872290;

    uint256 constant IC26x =
        11525629913109296967959183393964549768747692615404212892337575203974334462608;
    uint256 constant IC26y =
        5878533926249768910424982091808268978504398559551627740747771443356632204094;

    uint256 constant IC27x =
        20002913036260116795212867763287810702267097115043025643887673607407969655268;
    uint256 constant IC27y =
        558044083063510890683196204447038476167425982867815116695618624970741997206;

    uint256 constant IC28x =
        1455294901156212354915725105606392225281168194446399988016218564927279753087;
    uint256 constant IC28y =
        9265622034244521221313827609264536748455417116388928838433754993889267207246;

    uint256 constant IC29x =
        8492175542357327224267566257065741731581922730718422493423394842590799449908;
    uint256 constant IC29y =
        8652814149387146191572014893807964965786548948516478706728617370415006735859;

    uint256 constant IC30x =
        12945456920857623223065695579160061866066134561035673979412297019686810647455;
    uint256 constant IC30y =
        20767400459502682189711659499059158554474212104305263580689920062950451183406;

    uint256 constant IC31x =
        12452000809319676201338573627410389364483992019034748753979874328000765644471;
    uint256 constant IC31y =
        9324011591882185536935801579598120405827218960543046741826186886199801685754;

    uint256 constant IC32x =
        15117130650862375233097846251624547965782822125792934165533788502663832777561;
    uint256 constant IC32y =
        7327519711005978007931795867820761677338128328833770281221248240709259394804;

    uint256 constant IC33x =
        16999413180918098033592863681849974488877027642144119408578586494899105578070;
    uint256 constant IC33y =
        8919753301308754243845409919393688372970831151960044671687393095909635721078;

    uint256 constant IC34x =
        15649719163199433869832445862913171035095625562338955896073041648298851160839;
    uint256 constant IC34y =
        9189039051945082657627610414917017925248648316171459951020666357043783770327;

    // Memory data
    uint16 constant pVk = 0;
    uint16 constant pPairing = 128;

    uint16 constant pLastMem = 896;

    function verifyProof(
        uint[2] calldata _pA,
        uint[2][2] calldata _pB,
        uint[2] calldata _pC,
        uint[34] calldata _pubSignals
    ) public view returns (bool) {
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

                g1_mulAccC(
                    _pVk,
                    IC10x,
                    IC10y,
                    calldataload(add(pubSignals, 288))
                )

                g1_mulAccC(
                    _pVk,
                    IC11x,
                    IC11y,
                    calldataload(add(pubSignals, 320))
                )

                g1_mulAccC(
                    _pVk,
                    IC12x,
                    IC12y,
                    calldataload(add(pubSignals, 352))
                )

                g1_mulAccC(
                    _pVk,
                    IC13x,
                    IC13y,
                    calldataload(add(pubSignals, 384))
                )

                g1_mulAccC(
                    _pVk,
                    IC14x,
                    IC14y,
                    calldataload(add(pubSignals, 416))
                )

                g1_mulAccC(
                    _pVk,
                    IC15x,
                    IC15y,
                    calldataload(add(pubSignals, 448))
                )

                g1_mulAccC(
                    _pVk,
                    IC16x,
                    IC16y,
                    calldataload(add(pubSignals, 480))
                )

                g1_mulAccC(
                    _pVk,
                    IC17x,
                    IC17y,
                    calldataload(add(pubSignals, 512))
                )

                g1_mulAccC(
                    _pVk,
                    IC18x,
                    IC18y,
                    calldataload(add(pubSignals, 544))
                )

                g1_mulAccC(
                    _pVk,
                    IC19x,
                    IC19y,
                    calldataload(add(pubSignals, 576))
                )

                g1_mulAccC(
                    _pVk,
                    IC20x,
                    IC20y,
                    calldataload(add(pubSignals, 608))
                )

                g1_mulAccC(
                    _pVk,
                    IC21x,
                    IC21y,
                    calldataload(add(pubSignals, 640))
                )

                g1_mulAccC(
                    _pVk,
                    IC22x,
                    IC22y,
                    calldataload(add(pubSignals, 672))
                )

                g1_mulAccC(
                    _pVk,
                    IC23x,
                    IC23y,
                    calldataload(add(pubSignals, 704))
                )

                g1_mulAccC(
                    _pVk,
                    IC24x,
                    IC24y,
                    calldataload(add(pubSignals, 736))
                )

                g1_mulAccC(
                    _pVk,
                    IC25x,
                    IC25y,
                    calldataload(add(pubSignals, 768))
                )

                g1_mulAccC(
                    _pVk,
                    IC26x,
                    IC26y,
                    calldataload(add(pubSignals, 800))
                )

                g1_mulAccC(
                    _pVk,
                    IC27x,
                    IC27y,
                    calldataload(add(pubSignals, 832))
                )

                g1_mulAccC(
                    _pVk,
                    IC28x,
                    IC28y,
                    calldataload(add(pubSignals, 864))
                )

                g1_mulAccC(
                    _pVk,
                    IC29x,
                    IC29y,
                    calldataload(add(pubSignals, 896))
                )

                g1_mulAccC(
                    _pVk,
                    IC30x,
                    IC30y,
                    calldataload(add(pubSignals, 928))
                )

                g1_mulAccC(
                    _pVk,
                    IC31x,
                    IC31y,
                    calldataload(add(pubSignals, 960))
                )

                g1_mulAccC(
                    _pVk,
                    IC32x,
                    IC32y,
                    calldataload(add(pubSignals, 992))
                )

                g1_mulAccC(
                    _pVk,
                    IC33x,
                    IC33y,
                    calldataload(add(pubSignals, 1024))
                )

                g1_mulAccC(
                    _pVk,
                    IC34x,
                    IC34y,
                    calldataload(add(pubSignals, 1056))
                )

                // -A
                mstore(_pPairing, calldataload(pA))
                mstore(
                    add(_pPairing, 32),
                    mod(sub(q, calldataload(add(pA, 32))), q)
                )

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

                let success := staticcall(
                    sub(gas(), 2000),
                    8,
                    _pPairing,
                    768,
                    _pPairing,
                    0x20
                )

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

            checkField(calldataload(add(_pubSignals, 1088)))

            // Validate all evaluations
            let isValid := checkPairing(_pA, _pB, _pC, _pubSignals, pMem)

            mstore(0, isValid)
            return(0, 0x20)
        }
    }
}
