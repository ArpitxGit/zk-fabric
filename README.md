> Achieving full privacy preservation computation (encrypted computation) based on OT and garbled circuit in [6G Native Trustworthiness](https://www-file.huawei.com/-/media/corp2020/pdf/publications/huawei-research/issue2/6g-native-trustworthiness-en.pdf?la=en)

# ZK-FABRIC

zk-Fabric allows a cluster of verifiers to online, anonymously, and jointly compute a succinct digest of garbled circuits C which is prepared by a prover, who also practices the partitioning of the garbled circuit and randomly dispatches segments of them to a publicly accessible repository, i.e. the blockchain or a web portal. The goal is to build a more comprehensive public verification system which can validate more complex statements than other technologies that can only perform a monolithic verification, in other words, with which a verification can only conduct a single hashed value in an arithmetic circuit at a time.

# Claims

## Security Evaluation

zk-Fabric can maintain privacy against the semi-honest threat model (Note: zk-Fabric may not be sufficient in protection against the "Malicious" model). We can formalize this using a generalized Fiat-Shamir's secret sharing scheme, which defines a -secure n-party protocol and packs _l_ secrets into a single polynomial. One can run a joint computation for all inputs by just sending a constant number of field elements to the prover. As a result of packing _l_ secrets into a single polynomial, we can reduce the security bound t of zkFabric with multiple verifiers as _t = (n-1)/2_ to _t’ = t - l + 1_. In zk-Fabric, OT is a very useful building block in achieving protection against semi-honest participants.

## Computational Efficiency

zk-Fabric can achieve efficiency with two key refinements. First, we employ the Karnaugh Map technique to reduce the number of logical gates with a simplified expression. Second, we build garbled circuits with partitions by tightly integrating the verification procedure with a multi-party OT scheme. This reduces computational costs on the verifiers' side compared with native approaches.

_Notice: security definition and efficiency requirement immediately imply that the hash algorithm used to compute the succinct digest must be collision resistant._
