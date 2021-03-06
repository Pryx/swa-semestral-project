package cz.cvut.swa.bazaar.productservice.data

import org.springframework.data.annotation.Id
import org.springframework.data.mongodb.repository.MongoRepository
import java.math.BigDecimal
import java.time.LocalDateTime

data class Product(

        @Id val id: String? = null,
        val title: String,
        val description: String,
        val price: BigDecimal,
        val sellerId: Long,
        var status: ProductStatus = ProductStatus.AVAILABLE,
        val postedDatetime: LocalDateTime = LocalDateTime.now()

)

enum class ProductStatus {

    AVAILABLE,
    SOLD,
    DELETED

}

interface ProductRepository : MongoRepository<Product, String>