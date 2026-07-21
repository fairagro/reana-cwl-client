# REANA CWL Client
[![🦆 Continuous Integration](https://github.com/fairagro/reana-cwl-client/actions/workflows/ci.yaml/badge.svg)](https://github.com/fairagro/reana/actions/workflows/ci.yaml) ![Crates.io License](https://img.shields.io/crates/l/reana) 
![Crates.io Version](https://img.shields.io/crates/v/reana) ![Crates.io Total Downloads](https://img.shields.io/crates/d/reana)

Client library to use REANA in Rust based on the [commonwl]([github.com/fairagro/commonwl](https://github.com/fairagro/commonwl)) CWL parsing library.
The REANA CWL Client Library is used by [SciWIn](https://github.com/fairagro/sciwin) to communicate to REANA instances.

## REANA
[REANA](https://reanahub.io/) is a reproducible research data analysis platform developed by CERN and used by FAIRagro & SciWIn. REANA is capable of running Workflows of different kinds of languages at scale on high performance infrastructure. Namely Common Workflow Language (CWL), Snakemake, Yadage and Serial.