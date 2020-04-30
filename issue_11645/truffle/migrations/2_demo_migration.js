var Demo = artifacts.require("./Demo.sol");

module.exports = function(deployer) {
    // Demo is the contract's name
    deployer.deploy(Demo);
};
