#[allow(unused_imports)]
use super::*;

impl Bintf_T for () {
    fn write_to_js(&self, _writer: &mut dyn std::io::Write, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<(), Bintf_Err>
    where
        Self: Sized,
    {
        Ok(())
    }

    fn read_from_js(_reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        Ok(())
    }

    fn write_to_buf(&self, _writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        panic!("() :: write_to_buf")
    }

    fn read_from_buf(_reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        panic!("() :: read_from_buf")
    }
}

impl<T> Bintf_T for std::marker::PhantomData<T> {
    fn write_to_js(&self, _writer: &mut dyn std::io::Write, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<(), Bintf_Err>
    where
        Self: Sized,
    {
        Ok(())
    }

    fn read_from_js(_reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        Ok(PhantomData::default())
    }

    fn write_to_buf(&self, _writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        panic!("PhantomData :: write_to_buf")
    }

    fn read_from_buf(_reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        panic!("PhantomData :: write_to_buf")
    }
}

impl<T0: Bintf_T, T1: Bintf_T> Bintf_T for (T0, T1) {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        self.0.write_to_js(writer, _non_transfers, _transfers)?;
        self.1.write_to_js(writer, _non_transfers, _transfers)?;

        Ok(())
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<(T0, T1), Bintf_Err> {
        let t0 = <T0 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t1 = <T1 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        Ok((t0, t1))
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        self.0.write_to_buf(writer)?;
        self.1.write_to_buf(writer)?;

        Ok(())
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<(T0, T1), Bintf_Err> {
        let t0 = <T0 as Bintf_T>::read_from_buf(reader)?;
        let t1 = <T1 as Bintf_T>::read_from_buf(reader)?;
        Ok((t0, t1))
    }
}

impl<T0: Bintf_T, T1: Bintf_T, T2: Bintf_T> Bintf_T for (T0, T1, T2) {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        self.0.write_to_js(writer, _non_transfers, _transfers)?;
        self.1.write_to_js(writer, _non_transfers, _transfers)?;
        self.2.write_to_js(writer, _non_transfers, _transfers)?;
        Ok(())
    }

    fn read_from_js(
        reader: &mut dyn std::io::Read,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(T0, T1, T2), Bintf_Err> {
        let t0 = <T0 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t1 = <T1 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t2 = <T2 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        Ok((t0, t1, t2))
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        self.0.write_to_buf(writer)?;
        self.1.write_to_buf(writer)?;
        self.2.write_to_buf(writer)?;
        Ok(())
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<(T0, T1, T2), Bintf_Err> {
        let t0 = <T0 as Bintf_T>::read_from_buf(reader)?;
        let t1 = <T1 as Bintf_T>::read_from_buf(reader)?;
        let t2 = <T2 as Bintf_T>::read_from_buf(reader)?;
        Ok((t0, t1, t2))
    }
}

impl<T0: Bintf_T, T1: Bintf_T, T2: Bintf_T, T3: Bintf_T> Bintf_T for (T0, T1, T2, T3) {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        self.0.write_to_js(writer, _non_transfers, _transfers)?;
        self.1.write_to_js(writer, _non_transfers, _transfers)?;
        self.2.write_to_js(writer, _non_transfers, _transfers)?;
        self.3.write_to_js(writer, _non_transfers, _transfers)?;
        Ok(())
    }

    fn read_from_js(
        reader: &mut dyn std::io::Read,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(T0, T1, T2, T3), Bintf_Err> {
        let t0 = <T0 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t1 = <T1 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t2 = <T2 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t3 = <T3 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        Ok((t0, t1, t2, t3))
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        self.0.write_to_buf(writer)?;
        self.1.write_to_buf(writer)?;
        self.2.write_to_buf(writer)?;
        self.3.write_to_buf(writer)?;
        Ok(())
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<(T0, T1, T2, T3), Bintf_Err> {
        let t0 = <T0 as Bintf_T>::read_from_buf(reader)?;
        let t1 = <T1 as Bintf_T>::read_from_buf(reader)?;
        let t2 = <T2 as Bintf_T>::read_from_buf(reader)?;
        let t3 = <T3 as Bintf_T>::read_from_buf(reader)?;
        Ok((t0, t1, t2, t3))
    }
}

impl<T0: Bintf_T, T1: Bintf_T, T2: Bintf_T, T3: Bintf_T, T4: Bintf_T> Bintf_T for (T0, T1, T2, T3, T4) {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        self.0.write_to_js(writer, _non_transfers, _transfers)?;
        self.1.write_to_js(writer, _non_transfers, _transfers)?;
        self.2.write_to_js(writer, _non_transfers, _transfers)?;
        self.3.write_to_js(writer, _non_transfers, _transfers)?;
        self.4.write_to_js(writer, _non_transfers, _transfers)?;
        Ok(())
    }

    fn read_from_js(
        reader: &mut dyn std::io::Read,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(T0, T1, T2, T3, T4), Bintf_Err> {
        let t0 = <T0 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t1 = <T1 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t2 = <T2 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t3 = <T3 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t4 = <T4 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        Ok((t0, t1, t2, t3, t4))
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        self.0.write_to_buf(writer)?;
        self.1.write_to_buf(writer)?;
        self.2.write_to_buf(writer)?;
        self.3.write_to_buf(writer)?;
        self.4.write_to_buf(writer)?;
        Ok(())
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<(T0, T1, T2, T3, T4), Bintf_Err> {
        let t0 = <T0 as Bintf_T>::read_from_buf(reader)?;
        let t1 = <T1 as Bintf_T>::read_from_buf(reader)?;
        let t2 = <T2 as Bintf_T>::read_from_buf(reader)?;
        let t3 = <T3 as Bintf_T>::read_from_buf(reader)?;
        let t4 = <T4 as Bintf_T>::read_from_buf(reader)?;
        Ok((t0, t1, t2, t3, t4))
    }
}

impl<T0: Bintf_T, T1: Bintf_T, T2: Bintf_T, T3: Bintf_T, T4: Bintf_T, T5: Bintf_T> Bintf_T for (T0, T1, T2, T3, T4, T5) {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        self.0.write_to_js(writer, _non_transfers, _transfers)?;
        self.1.write_to_js(writer, _non_transfers, _transfers)?;
        self.2.write_to_js(writer, _non_transfers, _transfers)?;
        self.3.write_to_js(writer, _non_transfers, _transfers)?;
        self.4.write_to_js(writer, _non_transfers, _transfers)?;
        self.5.write_to_js(writer, _non_transfers, _transfers)?;
        Ok(())
    }

    fn read_from_js(
        reader: &mut dyn std::io::Read,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(T0, T1, T2, T3, T4, T5), Bintf_Err> {
        let t0 = <T0 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t1 = <T1 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t2 = <T2 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t3 = <T3 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t4 = <T4 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t5 = <T5 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        Ok((t0, t1, t2, t3, t4, t5))
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        self.0.write_to_buf(writer)?;
        self.1.write_to_buf(writer)?;
        self.2.write_to_buf(writer)?;
        self.3.write_to_buf(writer)?;
        self.4.write_to_buf(writer)?;
        self.5.write_to_buf(writer)?;
        Ok(())
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<(T0, T1, T2, T3, T4, T5), Bintf_Err> {
        let t0 = <T0 as Bintf_T>::read_from_buf(reader)?;
        let t1 = <T1 as Bintf_T>::read_from_buf(reader)?;
        let t2 = <T2 as Bintf_T>::read_from_buf(reader)?;
        let t3 = <T3 as Bintf_T>::read_from_buf(reader)?;
        let t4 = <T4 as Bintf_T>::read_from_buf(reader)?;
        let t5 = <T5 as Bintf_T>::read_from_buf(reader)?;
        Ok((t0, t1, t2, t3, t4, t5))
    }
}

impl<T0: Bintf_T, T1: Bintf_T, T2: Bintf_T, T3: Bintf_T, T4: Bintf_T, T5: Bintf_T, T6: Bintf_T> Bintf_T for (T0, T1, T2, T3, T4, T5, T6) {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        self.0.write_to_js(writer, _non_transfers, _transfers)?;
        self.1.write_to_js(writer, _non_transfers, _transfers)?;
        self.2.write_to_js(writer, _non_transfers, _transfers)?;
        self.3.write_to_js(writer, _non_transfers, _transfers)?;
        self.4.write_to_js(writer, _non_transfers, _transfers)?;
        self.5.write_to_js(writer, _non_transfers, _transfers)?;
        self.6.write_to_js(writer, _non_transfers, _transfers)?;
        Ok(())
    }

    fn read_from_js(
        reader: &mut dyn std::io::Read,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(T0, T1, T2, T3, T4, T5, T6), Bintf_Err> {
        let t0 = <T0 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t1 = <T1 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t2 = <T2 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t3 = <T3 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t4 = <T4 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t5 = <T5 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        let t6 = <T6 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        Ok((t0, t1, t2, t3, t4, t5, t6))
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        self.0.write_to_buf(writer)?;
        self.1.write_to_buf(writer)?;
        self.2.write_to_buf(writer)?;
        self.3.write_to_buf(writer)?;
        self.4.write_to_buf(writer)?;
        self.5.write_to_buf(writer)?;
        self.6.write_to_buf(writer)?;
        Ok(())
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<(T0, T1, T2, T3, T4, T5, T6), Bintf_Err> {
        let t0 = <T0 as Bintf_T>::read_from_buf(reader)?;
        let t1 = <T1 as Bintf_T>::read_from_buf(reader)?;
        let t2 = <T2 as Bintf_T>::read_from_buf(reader)?;
        let t3 = <T3 as Bintf_T>::read_from_buf(reader)?;
        let t4 = <T4 as Bintf_T>::read_from_buf(reader)?;
        let t5 = <T5 as Bintf_T>::read_from_buf(reader)?;
        let t6 = <T6 as Bintf_T>::read_from_buf(reader)?;
        Ok((t0, t1, t2, t3, t4, t5, t6))
    }
}
