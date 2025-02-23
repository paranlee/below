// Copyright (c) Facebook, Inc. and its affiliates.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

impl HasRenderConfigForDump for model::SystemModel {}

pub struct System {
    opts: GeneralOpt,
    fields: Vec<SystemField>,
}

impl System {
    pub fn new(opts: &GeneralOpt, fields: Vec<SystemField>) -> Self {
        Self {
            opts: opts.to_owned(),
            fields,
        }
    }
}

impl Dumper for System {
    fn dump_model(
        &self,
        ctx: &CommonFieldContext,
        model: &model::Model,
        output: &mut dyn Write,
        round: &mut usize,
        comma_flag: bool,
    ) -> Result<IterExecResult> {
        match self.opts.output_format {
            Some(OutputFormat::Raw) | None => write!(
                output,
                "{}",
                print::dump_raw(
                    &self.fields,
                    ctx,
                    &model.system,
                    *round,
                    self.opts.repeat_title,
                    self.opts.disable_title,
                    self.opts.raw
                )
            )?,
            Some(OutputFormat::Csv) => write!(
                output,
                "{}",
                print::dump_csv(
                    &self.fields,
                    ctx,
                    &model.system,
                    *round,
                    self.opts.disable_title,
                    self.opts.raw
                )
            )?,
            Some(OutputFormat::Tsv) => write!(
                output,
                "{}",
                print::dump_tsv(
                    &self.fields,
                    ctx,
                    &model.system,
                    *round,
                    self.opts.disable_title,
                    self.opts.raw
                )
            )?,
            Some(OutputFormat::KeyVal) => write!(
                output,
                "{}",
                print::dump_kv(&self.fields, ctx, &model.system, self.opts.raw)
            )?,
            Some(OutputFormat::Json) => {
                let par = print::dump_json(&self.fields, ctx, &model.system, self.opts.raw);
                if comma_flag {
                    write!(output, ",{}", par.to_string())?;
                } else {
                    write!(output, "{}", par.to_string())?;
                }
            }
        };

        *round += 1;

        Ok(IterExecResult::Success)
    }
}
